use futures::executor::block_on;
use masonry::{widget::Button, PointerButton};
use sea_orm::{entity::prelude::*, Database, DatabaseConnection};

mod entity; 
use entity::projects;
use xilem::{view::{button, flex, Axis, MainAxisAlignment}, WidgetView, Xilem};
use xilem_core::MessageResult;

async fn connect_database(url: &str) -> Result<DatabaseConnection, DbErr> {
    
    let db: DatabaseConnection = Database::connect(url).await?;
    Ok(db)
}

async fn get_projects(database: DatabaseConnection) -> Result<Vec<projects::Model>, DbErr> {
    let projects: Vec<projects::Model> = projects::Entity::find().all(&database).await?;
    Ok(projects)
}

fn data_add_projects(data: &mut AppData) {
    let res: Result<Vec<projects::Model>, DbErr> = block_on(get_projects(data.database.clone()));
    if res.is_err() {
        panic!("{}", res.unwrap_err());
    }
    let projects: Vec<projects::Model> = res.unwrap();
    data.projects.clear();
    for project in projects {
        println!("{:?}", project);
        data.projects.push(project.name);
    }
}

fn app_logic(data: &mut AppData) -> impl WidgetView<AppData> {
    // On crée un bouton
    data_add_projects(data);
    // Créé un vecteur de boutons pour chaque projet

    let mut buttons: Vec<Box<dyn for<'a> Fn(&'a mut AppData, PointerButton) -> MessageResult<()>>> = Vec::new();
    
    for project in &data.projects {
        let project = project.clone();
        bu
    }

    flex(buttons)
    .direction(Axis::Horizontal)
    .main_axis_alignment(MainAxisAlignment::Center)
}

struct AppData {
    database: DatabaseConnection,
    projects: Vec<String>
}

fn main() {
    const DATABASE_URL: &str = "mysql://root:root@localhost:3306/rajecto";

    let res: Result<DatabaseConnection, DbErr> = block_on(connect_database(DATABASE_URL));
    
    if res.is_err() {
        panic!("{}", res.unwrap_err());
    }

    let data = AppData {
        database: res.unwrap(),
        projects: Vec::new()
    };

    // Lance l'application
    let app = Xilem::new(data, app_logic);

    app.run_windowed("rajecto".into()).unwrap();
}