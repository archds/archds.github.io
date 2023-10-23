use crate::types::{Contact, Experience, Language, Link, LinkType, Skill, TextValue};

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

pub fn get_contacts() -> Vec<Contact> {
    return vec![
        Contact {
            link: "https://github.com/archds".to_string(),
            name: "GitHub".to_string(),
            _type: None,
        },
        Contact {
            link: "dsalekseev@gmail.com".to_string(),
            name: "Email".to_string(),
            _type: Some(LinkType::Email),
        },
    ];
}

pub fn get_skill_info() -> Vec<Language> {
    return vec![
        Language {
            logo: Some("https://icongr.am/devicon/python-original.svg?size=32&color=676767".to_string()),
            title: "Python".to_string(),
            skills: vec![
                Skill {
                    title: "Tools".to_string(),
                    items: vec![
                        TextValue::Link(Link {
                            text: "Django".to_string(),
                            url: "https://www.djangoproject.com/".to_string(),
                            _type: None,
                        }),
                        TextValue::Link(Link {
                            text: "FastAPI".to_string(),
                            url: "https://fastapi.tiangolo.com/".to_string(),
                            _type: None,
                        }),
                        TextValue::Link(Link {
                            text: "Pydantic".to_string(),
                            url: "https://docs.pydantic.dev/".to_string(),
                            _type: None,
                        }),
                    ],
                },
                Skill {
                    title: "Projects".to_string(),
                    items: vec![TextValue::Link(Link {
                        text: "Pyferret".to_string(),
                        url: "https://github.com/archds/pyferret".to_string(),
                        _type: None,
                    })],
                },
            ],
            description: vec![
                "Python is my primary developing tool. I like Python because it's so easy to read and write. Its clean 
                and simple syntax makes coding a breeze, and I find it great for variety of tasks. 
                The huge and active Python community means I'm never short 
                on resources and help when I need it. Python is very flexible and with good philosophy you can
                make reliable tools and products".to_string(),
            ]
        },
        Language {
            logo: Some("https://upload.wikimedia.org/wikipedia/commons/d/d5/Rust_programming_language_black_logo.svg".to_string()),
            title: "Rust".to_string(),
            skills: vec![
                Skill {
                    title: "Tools".to_string(),
                    items: vec![
                        TextValue::Text("Serde".to_string()),
                        TextValue::Link(Link {
                            text: "Yew".to_string(),
                            url: "https://yew.rs/".to_string(),
                            _type: None,
                        }),
                    ],
                },
                Skill {
                    title: "Projects".to_string(),
                    items: vec![TextValue::Link(Link {
                        text: "Chickapi".to_string(),
                        url: "https://github.com/archds/chikapi".to_string(),
                        _type: None,
                    })],
                },
            ],
            description: vec![
                "I really like Rust for several reasons. First and foremost, it's a language 
                that prioritizes safety and performance. Its strict compiler catches many common programming 
                errors at compile time, which saves me from dealing with hard-to-debug runtime issues. At last, but not least,
                I know Rust as parent for many high-performance and reliable developing tools, such as Pydantic and Ruff, and that
                gives me inspiration to work with.".to_string(),
            ]
        },
        Language {
            logo: Some("https://icongr.am/jam/wrench-f.svg?size=36&color=676767".to_string()),
            title: "Other".to_string(),
            skills: vec![
                Skill {
                    title: "CI/CD".to_string(),
                    items: vec![
                        TextValue::Text("Docker".to_string()),
                        TextValue::Text("Docker Compose".to_string()),
                        TextValue::Text("GitHub Actions".to_string()),
                    ],
                },
                Skill {
                    title: "Task management/Work".to_string(),
                    items: vec![
                        TextValue::Text("YouTrack".to_string()),
                        TextValue::Text("VSCode".to_string()),
                        TextValue::Text("Agile".to_string()),
                    ],
                },
            ],
            description: vec![
                "Despite as I like programming, I see how important other things for building a robust systems.
                Some tools are industry standard and not giving them enough attention would be a mistake.
                Also social interaction and task-management tools is important too, so I prefer not giving that
                part of work aside.".to_string(),
            ]
        },
    ];
}

pub fn get_experiences() -> Vec<Experience> {
    return vec![Experience {
        employer: "Someone".to_string(),
        from_age: "2020".to_string(),
        to_age: "2021".to_string(),
        job_duties: "Design, develop, and maintain server-side applications and infrastructure.
        Keep up-to-date with industry trends and emerging technologies to recommend and implement improvements.
        Optimize application performance, troubleshoot and debug issues, and implement solutions.
        Implement and maintain RESTful APIs for data exchange with front-end applications and third-party services.".to_string(),
        skills: vec![
            "PHP".to_string(),
            "Python".to_string(),
            "Test".to_string(),
            "Docker".to_string(),
        ]
    }];
}
