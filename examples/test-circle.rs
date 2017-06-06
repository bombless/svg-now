extern crate svg_now;
extern crate svg;

fn main() {
    let parsing = svg::parse(r#"
      <svg width="30" height="30" viewBox="0 0 30 30" xmlns="http://www.w3.org/2000/svg">
		<circle cx="15" cy="15" r="5" fill="yellow" />
	</svg>
    "#).unwrap();
  svg_now::render((100, 100), parsing);
}
