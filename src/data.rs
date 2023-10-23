use crate::types::ApplicationData;

pub const ABOUT_ME: [(&str, &str); 3] = [
    (
        "Backend Development",
        "I specialize in backend development, where my passion for building robust and efficient software solutions comes to life. 
        I thrive in collaborative team environments, where I can contribute my technical expertise and work collectively to achieve project goals. 
        My enthusiasm for learning and adapting to new technologies enables me to quickly grasp unfamiliar concepts and stay up-to-date in 
        the ever-evolving field of software development."
    ),
    (
        "Professional Experience",
        "Throughout my career, I've had the privilege of working on a variety of projects, from optimizing, including those involving 
        Event Sourcing and Domain-Driven Design (DDD). 
        I take pride in my ability to architect well-structured and efficient backend systems that meet the needs of both clients and end-users."
    ),
    (
        "Outside of Work",
        "In my free time, I lead an active lifestyle, participating in various sport activity. 
        Engaging in physical activities not only keeps me fit but also provides a healthy outlet for stress and creativity. 
        Additionally, I enjoy gaming as a way to unwind and explore virtual worlds, fostering problem-solving skills and 
        strategic thinking."
    )
];

const DATA_RAW: &str = include_str!("data.json");

pub fn get_data() -> Result<ApplicationData, serde_json::Error> {
    serde_json::from_str::<ApplicationData>(DATA_RAW)
}
