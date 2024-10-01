use futures::executor::block_on;
use sea_orm::{entity::prelude::*, Database, DatabaseConnection};

mod entity; 
use entity::projects;
use xilem::{view::{button, flex, Button}, MasonryView, Xilem};

async fn connect_database(url: &str) -> Result<DatabaseConnection, DbErr> {
    
    let db: DatabaseConnection = Database::connect(url).await?;
    Ok(db)
}

async fn get_projects(database: DatabaseConnection) -> Result<Vec<projects::Model>, DbErr> {
    let projects: Vec<projects::Model> = projects::Entity::find().all(&database).await?;
    Ok(projects)
}

fn button_action(data: &mut AppData) {
    let res: Result<Vec<projects::Model>, DbErr> = block_on(get_projects(data.database.clone()));
    if res.is_err() {
        panic!("{}", res.unwrap_err());
    }
    let projects: Vec<projects::Model> = res.unwrap();
    for project in projects {
        println!("{:?}", project);
        data.projects.push(project.name);
    }
}

fn app_logic(data: &mut AppData) -> impl MasonryView<AppData> {
    // On crée un bouton
    let request: Button<fn(&mut AppData)> = button("Get projects", button_action);
    // Pour chaque data.projects, on crée un bouton
    let mut buttons: Vec<Button<fn(&mut AppData)>> = Vec::new();
    for project in &data.projects {
        buttons.push(button(project.clone(), |_| println!("Hello")));
    }

    flex((request, buttons))
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