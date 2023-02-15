#![allow(unused)]

mod models;

use std::path::Path;
use std::{future::Future, process::Output};
// use image::{ self, Rgba, save_buffer, ColorType };
use models::{ Building, Complex, Flat };
use serde_json;
use fantoccini::{ClientBuilder, Locator};

struct BuildingDetails {
    name: String,
    link: String
}

const TARGET: &str = "https://crm.metriks.ru/shahmatki/agent";

#[tokio::main]
async fn main() -> Result<(), fantoccini::error::CmdError> {
    let mut caps = serde_json::map::Map::new();
    let chrome_opts = serde_json::json!({
        "args": [
        "--headless",
         "--disable-gpu",
         "--window-size=1920,1080",
        ]
/*
        args: [
        '--disable-web-security',
        '--disable-features=IsolateOrigins,site-per-process',
        '--window-size=1920,1080',
        ], 
*/
    });

    caps.insert("goog:chromeOptions".to_string(), chrome_opts);

    let client = ClientBuilder::native()
                    .capabilities(caps)
                    .connect("http://localhost:4444")
                    .await
                    .expect("failed to connect to WebDriver");

    client.fullscreen_window().await.unwrap();

    client.goto(TARGET).await?;
    
    let container_class = ".main__item";

    let complexes = client.find_all(Locator::Css(container_class)).await.unwrap();

    for complex_block in complexes {
        let selector = format!("{container_class}-top {container_class}-name");
        let name = complex_block.find(Locator::Css(selector.as_str()))
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        println!("[WORKING ON COMPLEX] {}", name);
        
        let mut complex = Complex::new(name);

        let buildings_details = complex_block
            .find_all(Locator::Css("a"))
            .await
            .unwrap();

        for building_details in buildings_details {
            let building_link = building_details
                .prop("href")
                .await
                .unwrap()
                .ok_or(String::new())
                .unwrap();

            let building_name = building_details.text().await.unwrap();

            client.goto(&building_link).await?;

            let apartment_selector = ".chess__href:has(> .white)";

            client.wait().for_element(Locator::Css(apartment_selector)).await.unwrap();

            let apartments_elements = client.find_all(Locator::Css(apartment_selector)).await.unwrap();

            println!("[WORKING ON BUILDING] {}", building_name);
            
            let mut building = Building::new(building_name);

            for apartment_el in apartments_elements {
                /* let apartment_link = apartment_el.prop("href").await.unwrap().ok_or(String::new()).unwrap(); */

                apartment_el.click().await;

                let data_wrapper_selector = "#change-popup.change.visible";

                client.wait().for_element(Locator::Css(data_wrapper_selector)).await.unwrap();
                client.wait().for_element(Locator::Css("td#VIEW-ROOMTEXT")).await.unwrap();

                let data_wrapper = client.find(Locator::Css(data_wrapper_selector)).await.unwrap();

                /* tokio::time::sleep(tokio::time::Duration::from_millis(500)); */
                // let buffer = client.screenshot().await.unwrap();
                // save_buffer(&Path::new("screenshot.png"), &buffer, 1920, 1080, ColorType::Rgba16);
                // std::process::exit(1);

                let apartment = data_wrapper.find(Locator::Css("#VIEW-NUM"))
                    .await
                    .expect("Couldn't find el with selector: <#VIEW-NUM>")
                    .text()
                    .await
                    .expect("Couldn't find el with selector: <#VIEW-NUM>")
                    .parse()
                    .expect("Couldn't find el with selector: <#VIEW-NUM>");

                let is_apartment = data_wrapper.find(Locator::Css("#VIEW-TYPETEXT"))
                    .await
                    .expect("Couldn't find el with selector: <#VIEW-TYPETEXT>")
                    .text()
                    .await
                    .expect("Couldn't find el with selector: <#VIEW-TYPETEXT>").as_str() == "Квартира";

                let rooms = data_wrapper.find(Locator::Css("#VIEW-ROOMTEXT"))
                    .await
                    .expect("Couldn't find el with selector: <#VIEW-ROOMTEXT>")
                    .text()
                    .await
                    .expect("Couldn't find el with selector: <#VIEW-ROOMTEXT>");

                let area = data_wrapper.find(Locator::Css("#VIEW-AREA"))
                    .await
                    .expect("Couldn't find el with selector: <#VIEW-AREA>")
                    .text()
                    .await
                    .expect("Couldn't find el with selector: <#VIEW-AREA>");

                let plan = data_wrapper.find(Locator::Css(".layout__img img"))
                    .await
                    .expect("Couldn't find el with selector: <.layout__img img>")
                    .prop("src")
                    .await
                    .expect("Couldn't find el with selector: <.layout__img img>")
                    .ok_or(String::new())
                    .expect("Couldn't find el with selector: <.layout__img img>");

                let flat = Flat {
                    apartment,
                    rooms,
                    area,
                    plan
                };

                println!("{:#?}", flat);

                building.flats.push(flat);

                println!("[APARTMENT EXTRACTED] Num: {apartment}");
            }

            complex.buildings.push(building);
        }
    
        println!("{:#?}", complex);
    }

    client.close().await
}

// std::process::exit(1);

// let price: document.getElementById("VIEW-PRICEALL_CLR_TEXT")?.innerText?.replace("₽", "").replace(" ", ""),