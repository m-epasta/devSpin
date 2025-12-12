module executor

import term

// Color utility functions for better CLI readability

// Success messages (green with checkmark)
pub fn print_success(msg string) {
	println(term.green('  [OK] ${msg}'))
}

// Error messages (red with cross)
pub fn print_error(msg string) {
	eprintln(term.red('  [ERROR] ${msg}'))
}

// Warning messages (yellow with warning symbol)
pub fn print_warning(msg string) {
	println(term.yellow('  [WARN] ${msg}'))
}

// Info messages (cyan)
pub fn print_info(msg string) {
	println(term.cyan('  [INFO] ${msg}'))
}

// Highlighted/emphasized text (bright white/bold)
pub fn print_highlight(msg string) {
	println(term.bright_white(term.bold(msg)))
}

// Command text (bright magenta)
pub fn print_command(msg string) {
	println(term.bright_magenta('  $ ${msg}'))
}

// Prompt text (bright blue with prompt symbol)
pub fn print_prompt(msg string) {
	print(term.bright_blue('  > ${msg}'))
}

// File path (bright white)
pub fn format_path(path string) string {
	return term.bright_white(path)
}

// Format a number/count (bright cyan)
pub fn format_count(count int) string {
	return term.bright_cyan('${count}')
}

// Section header (bold bright blue)
pub fn print_header(msg string) {
	println('')
	println(term.bright_blue(term.bold('  === ${msg} ===')))
	println('')
}

// Separator line
pub fn print_separator() {
	println(term.gray('  ' + '-'.repeat(50)))
}

// Box drawing utilities
pub fn print_box_top(width int) {
	println(term.cyan('  +' + '-'.repeat(width) + '+'))
}

pub fn print_box_middle(content string, width int) {
	padding := width - content.len
	left_pad := padding / 2
	right_pad := padding - left_pad
	println(term.cyan('  |') + ' '.repeat(left_pad) + content + ' '.repeat(right_pad) +
		term.cyan('|'))
}

pub fn print_box_bottom(width int) {
	println(term.cyan('  +' + '-'.repeat(width) + '+'))
}

// Language labels (no icons)
pub fn get_lang_label(typ string) string {
	return match typ {
		'v' { 'V' }
		'rust', 'rs' { 'Rust' }
		'go' { 'Go' }
		'js', 'javascript' { 'JavaScript' }
		'ts', 'typescript' { 'TypeScript' }
		'python', 'py' { 'Python' }
		'json' { 'JSON' }
		'yaml', 'yml' { 'YAML' }
		'toml' { 'TOML' }
		else { typ.to_upper() }
	}
}

// Progress bar
pub fn print_progress(current int, total int, width int) {
	if total == 0 {
		return
	}
	filled := (current * width) / total
	empty := width - filled
	bar := term.green('#'.repeat(filled)) + term.gray('-'.repeat(empty))
	percent := (current * 100) / total
	print('\r  [${bar}] ${percent}%')
	if current == total {
		println('')
	}
}

// Bullet point list item
pub fn print_bullet(msg string) {
	println(term.gray('  * ') + msg)
}

// Numbered list item
pub fn print_numbered(num int, msg string) {
	println(term.gray('  ${num}. ') + msg)
}

// Dim/muted text
pub fn print_dim(msg string) {
	println(term.gray('  ${msg}'))
}

// Bold text
pub fn print_bold(msg string) {
	println(term.bold('  ${msg}'))
}

// Status badge
pub fn format_status(success bool) string {
	if success {
		return term.green('[PASS]')
	} else {
		return term.red('[FAIL]')
	}
}
