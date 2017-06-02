extern crate svg_now;
extern crate svg;

fn main() {
    println!("P3");
    println!("100 100");
    println!("255");
    let parsing = svg::parse(r#"
      <svg width="30" height="30" viewBox="0 0 30 30" xmlns="http://www.w3.org/2000/svg">
		<line x1="15" y1="0" x2="15" y2="30" stroke-width="2" stroke="black"/>
	</svg>
    "#).unwrap();
    for x in &parsing {
      //println!("{:?}", x)
    }
    //return;
    for (idx, x) in svg_now::render((100, 100), parsing).into_iter().enumerate() {
      if idx % 4 == 3 { continue }
      println!("{}", x)
    }
}
