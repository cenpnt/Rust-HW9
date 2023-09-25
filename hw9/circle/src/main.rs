use rand::Rng;
use std::io::Read;
use std::fs::File;
use std::io::Write;
use csv::{ReaderBuilder, Writer, Trim};

#[derive(Debug, PartialEq, Clone)]
struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}
#[derive(Debug, PartialEq, Clone)]
struct Layer {
    name: String,
    color: String,
    circles: Vec<Circle>,
}

fn main() {
    to_csv();
    convert_to_avg();
    to_html();
    min_max_html();
}

#[allow(dead_code)]
fn gen_layer<R: Rng>(name: String, color: String, rng: &mut R) -> Layer {
    let mut circle = Vec::new();
    let n = rng.gen_range(20..=50);

    for _ in 0..n {
        let x = rng.gen_range(-100.0..=100.0);
        let y = rng.gen_range(-100.0..=100.0);
        let radius = rng.gen_range(-10.0..=20.0);
        circle.push(Circle {
            x,
            y,
            radius,
        });
    }
    Layer {
        name,
        color,
        circles: circle
    }
}

#[allow(dead_code)]
fn gen_obj_layer_list<R: Rng>(rng: &mut R, n: i64) -> Vec<Layer> {
    let mut layer = Vec::new();

    for i in 0..n {
        let name = format!("Layer {}", i + 1);
        let color: i64 = rng.gen_range(0x00000000..=0xFFFFFFFF);
        let formatted_color = format!("#{:08x}", color);
        layer.push(gen_layer(name, formatted_color, rng));
        // vec![ Layer { name: Layer 1, color: #00000000, circles: vec![Circle { x: 1.0, y: 2.0, radius: 1.0 }, Circle { x: 5.0, y: 10.0, radius: 2.0 }] }
    }
    layer

}

#[test]
fn test_gen_layer_list() {
    let mut rng = rand::thread_rng();
    let n = 5;
    let layers = gen_obj_layer_list(&mut rng, n);
    
    assert_eq!(layers.len(), n.try_into().unwrap());
    
    for (i, layer) in layers.iter().enumerate() {
        assert_eq!(layers.len(), n.try_into().unwrap());
        
        for circle in &layer.circles {
            assert!(circle.x >= -100.0 && circle.x <= 100.0);
            assert!(circle.y >= -100.0 && circle.y <= 100.0);
            assert!(circle.radius >= -10.0 && circle.radius <= 20.0);
        }
        
        assert_eq!(layer.name, format!("Layer {}", i+1));
        assert_eq!(layer.color.chars().count(), 9);
    }
}

#[allow(dead_code)]
fn cal_average_area(layers : Vec<Layer>) -> Vec<(String, f64)> {
    let mut result = Vec::new();
    for layer in layers {
        let mut sum = 0.0;
        let mut count = 0.0;
        for circle in layer.circles {
            sum += std::f64::consts::PI * circle.radius * circle.radius;
            count += 1.0;
        }
        let avg = sum / count;
        result.push((layer.name, avg))
    }
    result
}

#[test]
fn test_cal_average_area() {
    let mut rng = rand::thread_rng();
    let color: i64 = rng.gen_range(0x00000000..=0xFFFFFFFF);
    let formatted_color = format!("#{:08x}", color);
    let layers = vec![
        Layer {
            name: String::from("Layer 1"),
            color: String::from(formatted_color.clone()),
            circles: vec![
                Circle { x: 1.0, y: 2.0, radius: 1.0 },
                Circle { x: 5.0, y: 10.0, radius: 2.0 },
            ],
        },
        Layer {
            name: String::from("Layer 2"),
            color: String::from(formatted_color.clone()),
            circles: vec![
                Circle { x: 4.0, y: 3.0, radius: 3.0 },
                Circle { x: 5.0, y: 10.0, radius: 4.0 },
            ],
        },
    ];

    let result = cal_average_area(layers);

    let expected = vec![
        (String::from("Layer 1"), (std::f64::consts::PI * 1.0 * 1.0 + std::f64::consts::PI * 2.0 * 2.0) / 2.0),
        (String::from("Layer 2"), (std::f64::consts::PI * 3.0 * 3.0 + std::f64::consts::PI * 4.0 * 4.0) / 2.0),
    ];

    for (calculated, expected) in result.iter().zip(expected.iter()) {
        assert_eq!(calculated, expected);
    }
}

fn write_layer_csv<W: Write>(writer: W, layers: Vec<Layer>) {
    let mut wtr = csv::Writer::from_writer(writer);

    for layer in layers {
        let mut records = Vec::new();

        records.push(layer.name);
        records.push(layer.color);

        let circle_str = layer.circles
        .iter()
        .map(|circle| format!("{:.1},{:.1},{:.1}", circle.x, circle.y, circle.radius))
        .collect::<Vec<String>>()
        .join("; ");
        
        records.push(circle_str);
        wtr.write_record(&records).unwrap();
    }
    wtr.flush().unwrap();
}

#[allow(dead_code)]
fn to_csv() {
    let mut rng = rand::thread_rng();
    let n = 5;
    let layers = gen_obj_layer_list(&mut rng, n);
    write_layer_csv(File::create("output.csv").unwrap(), layers);
}  
///////////////////////////////////////////
//2.2
#[allow(dead_code)]
fn read_csv<R: Read>(rdr: R) -> Vec<Layer> {
    let mut reader = ReaderBuilder::new()
        .delimiter(b',')
        .has_headers(false)
        .trim(Trim::All)
        .from_reader(rdr);
    let mut layers = vec![];
    
    for record in reader.records() {
        if let Ok(rec) = record {
            let name = rec[0].to_string();
            let color = rec[1].to_string();
            
            let circles_data: Vec<&str> = rec[2].split(';').map(|s| s.trim()).collect();
            let mut circles = vec![];
            
            for circle_str in circles_data {
                let parts: Vec<&str> = circle_str.split(',').collect();
                
                if parts.len() == 3 {
                    let x: f64 = parts[0].parse().unwrap();
                    let y: f64 = parts[1].parse().unwrap();
                    let radius: f64 = parts[2].parse().unwrap();
                    
                    circles.push(Circle { x, y, radius });
                }
            }
            
            layers.push(Layer { name, color, circles });
        }
    }
    layers
}

#[allow(dead_code)]
fn write_layer_avg<W: Write>(writer: W, avg_areas: Vec<(String, f64)>) {
    let mut wtr = Writer::from_writer(writer);

    for (name, avg_area) in avg_areas {
        wtr.write_record(&[name, avg_area.to_string()]).unwrap();
    }
    wtr.flush().unwrap();
}

#[allow(dead_code)]
fn convert_to_avg() {
    let file = File::open("output.csv").unwrap();
    let read = read_csv(file);
    let convert = cal_average_area(read);
    let result = write_layer_avg(File::create("output_converted.csv").unwrap(), convert);
    result
}

///////////////////////////////////////////
// 3.1
#[allow(dead_code)]
fn to_html() {
    let mut table = String::new();
    table.push_str("<!DOCTYPE html>
    <html>
        <head>
            <title>HTML Table</title>
            <style> table, th, td {
                border: 1px solid #000000;
                text-align: center;
                width: 50%;
                border-collapse: collapse; 
                }
            </style>
            <h1>Generate and Average</h1>
        </head>
        <body>
            <table>
                <thead>
                    <tr>
                        <th>Layer</th>
                        <th>Average Area</th>
                    </tr>
                </thead>
                <tbody>");  

    let mut rng = rand::thread_rng();
    let n = 5;
    let layers = gen_obj_layer_list(&mut rng, n);
    let avg_all = cal_average_area(layers.clone());

    for (element_in_layer, avg) in layers.iter().zip(avg_all.iter()) {
        table.push_str(&format!("<tr>
                                    <td>{}</td>
                                    <td>{:.2}</td>
                                </tr>", element_in_layer.name, avg.1)); //avg.1 = second element in tuple which is average not name
    }
    table.push_str("</tbody>
                    </table>
                </body>
            </html>"
                );
    
    let mut file = File::create("output.html").unwrap();
    file.write(table.as_bytes()).unwrap();

}

//3.2
fn min_max_html() {
    let mut table = String::new();
    table.push_str("<!DOCTYPE html>
    <html>
        <head>
            <title>HTML Table</title>
            <style> table, th, td {
                border: 1px solid #000000;
                text-align: center;
                width: 25%;
                border-collapse: collapse;
                margin: 20px;
                }
            </style>
            <h1>Max, Min, Average</h1>
        </head>
        <body>
            <table>
                <thead>
                    <tr>
                        <th>Layer</th>
                        <th>Maximum Area</th>
                        <th>Minimum Area</th>
                        <th>Average Area</th>
                    </tr>
                </thead>
                <tbody>");  

    let mut rng = rand::thread_rng();
    let n = 5;
    let layers = gen_obj_layer_list(&mut rng, n);
    let avg_all = cal_average_area(layers.clone());

    for (element_in_layers, avg) in layers.iter().zip(avg_all.iter()) {
        let mut max_area = std::f64::NEG_INFINITY; //It guaranteed to be less than or equal to area so that it can be compared when area is greater than max_area
        let mut min_area = std::f64::INFINITY; // It guaranteed to be greater than or equal to area so that it can be compared when area is less than min_area
        for circle in &element_in_layers.circles {
            let area = std::f64::consts::PI * circle.radius * circle.radius;
            if area > max_area {
                max_area = area;
            } else if area < min_area{
                min_area = area;
            }
        }
        table.push_str(&format!("<tr>
                                    <td>{}</td>
                                    <td>{:.2}</td>
                                    <td>{:.2}</td>
                                    <td>{:.2}</td>
                                </tr>", element_in_layers.name, max_area, min_area, avg.1)); //avg.1 = second element in tuple which is average not name
    }
    table.push_str("</tbody>
                    </table>
                </body>
            </html>"
                );
    
    let mut file = File::create("output_min_max.html").unwrap();
    file.write(table.as_bytes()).unwrap();
}