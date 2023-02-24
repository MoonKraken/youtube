import markdown

fn main() {
	text := '# Markdown Rocks!'
	output := markdown.to_html(text)
	println(output)
}
