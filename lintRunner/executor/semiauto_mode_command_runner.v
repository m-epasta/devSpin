module executor

import os
import term

// Semi-auto mode: Shows what will be done and asks for confirmation before each step
fn run_semi_auto_mode(dir_path string, file_name string, typ string, op Operation, auto_format bool) ! {
	// cd in the dir
	go_to_path(dir_path)

	operation_str := if op == .lint { 'linting' } else { 'formatting' }

	// Show header
	println('')
	println(term.bright_cyan(term.bold('+-------------------------------------------+')))
	println(term.bright_cyan(term.bold('|') + '           SEMI-AUTO MODE                 ' +
		term.bold('|')))
	println(term.bright_cyan(term.bold('+-------------------------------------------+')))
	println('')

	// Show operation details
	println(term.gray('  Operation: ') + term.bright_white(term.bold(operation_str.to_upper())))
	println(term.gray('  Language:  ') + term.bright_yellow(get_lang_label(typ)))
	if file_name != '' {
		println(term.gray('  Target:    ') + term.bright_green(file_name))
	} else {
		println(term.gray('  Target:    ') + term.bright_blue('entire directory'))
	}
	println('')

	// Get the command that would be executed
	processed_cmd := choose_cmd_by_op_and_typ(op, typ, file_name)

	// Show command preview in a box
	println(term.yellow('  Command to execute:'))
	println(term.gray('  +' + '-'.repeat(processed_cmd.len + 4) + '+'))
	println(term.gray('  | ') + term.bright_white(processed_cmd) + term.gray(' |'))
	println(term.gray('  +' + '-'.repeat(processed_cmd.len + 4) + '+'))
	println('')

	// Ask for confirmation with colored prompt
	mut confirm := ''
	for {
		print(term.bright_magenta('  > ') + 'Proceed with ${operation_str}? ' + term.gray('[') +
			term.green('y') + term.gray('/') + term.red('n') + term.gray('] '))
		confirm = os.input('').to_lower().trim_space()
		if confirm in ['y', 'n', 'yes', 'no'] {
			break
		}
		println(term.yellow('  [WARN] Please enter "y" or "n".'))
	}

	if confirm in ['n', 'no'] {
		println('')
		println(term.yellow('  [INFO] Operation cancelled by user.'))
		println('')
		return
	}

	// Execute with progress indicator
	println('')
	println(term.cyan('  Executing...'))
	cmd_result := os.execute(processed_cmd)

	// Handle results based on operation type
	println('')
	if op == .format {
		if typ == 'v' {
			handle_v_formatting(cmd_result, file_name, auto_format)!
		} else {
			handle_generic_formatting(cmd_result, typ, file_name, auto_format)!
		}
	} else {
		// Linting
		if cmd_result.exit_code != 0 {
			println(term.red('  [ERROR] ${operation_str} errors found:'))
			if cmd_result.output.len > 0 {
				// Indent and colorize output
				for line in cmd_result.output.split('\n') {
					if line.len > 0 {
						println(term.gray('    | ') + line)
					}
				}
			}
			println('')
			return error('${operation_str} failed with exit code ${cmd_result.exit_code}')
		} else {
			println(term.green('  [OK] ${operation_str} passed!'))
		}
	}

	// Show summary
	show_summary(op, typ, file_name, cmd_result.exit_code == 0)
}

// Show a nice summary at the end
fn show_summary(op Operation, typ string, file_name string, success bool) {
	println('')
	if success {
		println(term.bright_green(term.bold('+-------------------------------------------+')))
		println(term.bright_green(term.bold('|') + '           OPERATION COMPLETE             ' +
			term.bold('|')))
		println(term.bright_green(term.bold('+-------------------------------------------+')))
	} else {
		println(term.bright_red(term.bold('+-------------------------------------------+')))
		println(term.bright_red(term.bold('|') + '           OPERATION FAILED               ' +
			term.bold('|')))
		println(term.bright_red(term.bold('+-------------------------------------------+')))
	}
	println('')
}

// Interactive menu for semi-auto mode with colors
fn show_semi_auto_menu(typ string, op Operation) string {
	operation_str := if op == .lint { 'Lint' } else { 'Format' }
	op_color := if op == .lint { term.bright_blue } else { term.bright_magenta }

	println('')
	println(term.bright_cyan('+-------------------------------------------+'))
	println(term.bright_cyan('|') + op_color('   ${operation_str} Options for ${typ}') +
		'               ' + term.bright_cyan('|'))
	println(term.bright_cyan('+-------------------------------------------+'))
	println('')
	println(term.gray('  [') + term.bright_green('1') + term.gray('] ') + 'Run on current directory')
	println(term.gray('  [') + term.bright_green('2') + term.gray('] ') + 'Run on specific file')
	println(term.gray('  [') + term.bright_yellow('3') + term.gray('] ') +
		'Show command preview only')
	println(term.gray('  [') + term.bright_red('4') + term.gray('] ') + 'Cancel')
	println('')

	mut choice := ''
	for {
		print(term.bright_magenta('  > ') + 'Select option ' + term.gray('[1-4]: '))
		choice = os.input('').trim_space()
		if choice in ['1', '2', '3', '4'] {
			break
		}
		println(term.yellow('  [WARN] Invalid option. Please enter 1, 2, 3, or 4.'))
	}

	return choice
}

// Get file path from user input with validation
fn get_file_from_user() string {
	print(term.bright_magenta('  > ') + 'Enter file path: ')
	file_path := os.input('').trim_space()

	if file_path.len == 0 {
		println(term.yellow('  [WARN] No file path provided.'))
		return ''
	}

	if !os.exists(file_path) {
		println(term.red('  [ERROR] File does not exist: ') + term.gray(file_path))
		return ''
	}

	println(term.green('  [OK] File found: ') + term.bright_white(file_path))
	return file_path
}
