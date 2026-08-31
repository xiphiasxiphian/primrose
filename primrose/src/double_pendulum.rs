use engine::{
    clock::Clock,
    glam::DVec2,
    jade::{
        ecs::{
            components::{
                renderable::RenderInfo,
                transform::{self, Transform},
            },
            query::Query,
            world::World,
        },
        scene::manager::GlobalResources,
    },
    proc_macros::Component,
};

#[derive(Clone, Copy, Debug)]
pub struct State
{
    pub theta1: f64,
    pub theta2: f64,
    pub omega1: f64,
    pub omega2: f64,
}

#[derive(Clone, Copy, Debug, Component)]
pub struct DoublePendulum
{
    pub state: State,
    pub l1: f64,
    pub l2: f64,
    pub m1: f64,
    pub m2: f64,
    pub g: f64,
    pub dampning: f64,
}

impl Default for DoublePendulum
{
    fn default() -> Self
    {
        DoublePendulum {
            state: State {
                theta1: 90.0_f64.to_radians(),
                theta2: 90.0_f64.to_radians(),
                omega1: 0.0,
                omega2: 0.0,
            },
            l1: 200.0,
            l2: 200.0,
            m1: 0.5,
            m2: 0.5,
            g: 1500.0,
            dampning: 0.0,
        }
    }
}

impl DoublePendulum
{
    fn derivatives(&self, s: State) -> (f64, f64, f64, f64)
    {
        let Self {
            state: _,
            l1,
            l2,
            m1,
            m2,
            g,
            dampning,
        } = *self;
        let State {
            theta1,
            theta2,
            omega1,
            omega2,
        } = s;

        let delta = theta1 - theta2;

        let n1 = (-g * (2.0 * m1 + m2) * theta1.sin())
            - (m2 * g * (theta1 - 2.0 * theta2).sin())
            - (2.0 * delta.sin() * m2 * (omega2.powi(2) * l2 + omega1.powi(2) * l1 * delta.cos()));
        let d1 = l1 * (2.0 * m1 + m2 - m2 * (2.0 * theta1 - 2.0 * theta2).cos());
        let alpha1 = n1 / d1;

        let n2 = 2.0
            * delta.sin()
            * (omega1.powi(2) * l1 * (m1 + m2) + g * (m1 + m2) * theta1.cos() + omega2.powi(2) * l2 * m2 * delta.cos());
        let d2 = l2 * (2.0 * m1 + m2 - m2 * (2.0 * theta1 - 2.0 * theta2).cos());
        let alpha2 = n2 / d2;

        let d_omega1 = alpha1 - dampning * omega1;
        let d_omega2 = alpha2 - dampning * omega2;

        (omega1, omega2, d_omega1, d_omega2)
    }

    pub fn step(&mut self, dt: f64)
    {
        let s = self.state;

        let (d_th1_1, d_th2_1, d_om1_1, d_om2_1) = self.derivatives(s);

        let s2 = State {
            theta1: s.theta1 + 0.5 * dt * d_th1_1,
            theta2: s.theta2 + 0.5 * dt * d_th2_1,
            omega1: s.omega1 + 0.5 * dt * d_om1_1,
            omega2: s.omega2 + 0.5 * dt * d_om2_1,
        };
        let (d_th1_2, d_th2_2, d_om1_2, d_om2_2) = self.derivatives(s2);

        let s3 = State {
            theta1: s.theta1 + 0.5 * dt * d_th1_2,
            theta2: s.theta2 + 0.5 * dt * d_th2_2,
            omega1: s.omega1 + 0.5 * dt * d_om1_2,
            omega2: s.omega2 + 0.5 * dt * d_om2_2,
        };
        let (d_th1_3, d_th2_3, d_om1_3, d_om2_3) = self.derivatives(s3);

        let s4 = State {
            theta1: s.theta1 + dt * d_th1_3,
            theta2: s.theta2 + dt * d_th2_3,
            omega1: s.omega1 + dt * d_om1_3,
            omega2: s.omega2 + dt * d_om2_3,
        };
        let (d_th1_4, d_th2_4, d_om1_4, d_om2_4) = self.derivatives(s4);

        self.state.theta1 += (dt / 6.0) * (d_th1_1 + 2.0 * d_th1_2 + 2.0 * d_th1_3 + d_th1_4);
        self.state.theta2 += (dt / 6.0) * (d_th2_1 + 2.0 * d_th2_2 + 2.0 * d_th2_3 + d_th2_4);
        self.state.omega1 += (dt / 6.0) * (d_om1_1 + 2.0 * d_om1_2 + 2.0 * d_om1_3 + d_om1_4);
        self.state.omega2 += (dt / 6.0) * (d_om2_1 + 2.0 * d_om2_2 + 2.0 * d_om2_3 + d_om2_4);
    }

    pub fn get_node_positions(&self, base: DVec2) -> (DVec2, DVec2)
    {
        let node1 = base + DVec2::new(self.state.theta1.sin(), self.state.theta1.cos()) * self.l1;

        let node2 = node1 + DVec2::new(self.state.theta2.sin(), self.state.theta2.cos()) * self.l2;

        (node1, node2)
    }
}

pub fn double_pendulum_system(
    query: Query<(&mut DoublePendulum, &mut RenderInfo, &Transform)>,
    globals: &mut GlobalResources,
)
{
    for (double_pen, render, transform) in query.iter()
    {
        for _ in 0..5
        {
            double_pen.step(globals.clock.dt() / 5.0)
        }

        let (node1, node2) = double_pen.get_node_positions(transform.pos);

        let node_color = [1.0, 0.0, 0.0, 1.0];
        let line_color = [0.0, 1.0, 0.0, 1.0];

        let node_size = 30.0;

        // render pendulum
        render.line(transform.pos, node1, 4.0, line_color);
        render.line(node1, node2, 4.0, line_color);

        render.circle(node1, node_size, node_color, 64);
        render.circle(node2, node_size, node_color, 64);
    }
}
