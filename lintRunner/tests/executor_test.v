module tests

import os
import executor

// ============== Enum Tests ==============

fn test_operation_lint_enum() {
	op := executor.Operation.lint
	assert op == .lint
}

fn test_operation_format_enum() {
	op := executor.Operation.format
	assert op == .format
}

fn test_mode_auto_enum() {
	mode := executor.Mode.auto
	assert mode == .auto
}

fn test_mode_semi_auto_enum() {
	mode := executor.Mode.semi_auto
	assert mode == .semi_auto
}

// ============== Options Parsing Tests ==============

fn test_parse_options_valid_lint() {
	opts := executor.parse_and_validate_options('auto', '', 'lint', false, '') or {
		assert false, 'Should not fail for valid options: ${err}'
		return
	}
	assert opts.operation == .lint
	assert opts.auto_format == false
}

fn test_parse_options_valid_format() {
	opts := executor.parse_and_validate_options('auto', '', 'format', true, '') or {
		assert false, 'Should not fail for valid options: ${err}'
		return
	}
	assert opts.operation == .format
	assert opts.auto_format == true
}

fn test_parse_options_semi_auto_mode() {
	opts := executor.parse_and_validate_options('semi-auto', '', 'lint', false, '') or {
		assert false, 'Should not fail: ${err}'
		return
	}
	// Just verify it doesn't fail - mode field is private
	assert opts.operation == .lint
}

fn test_parse_options_invalid_mode() {
	executor.parse_and_validate_options('invalid', '', 'lint', false, '') or {
		assert err.msg().contains('Invalid mode')
		return
	}
	assert false, 'Should have failed for invalid mode'
}

fn test_parse_options_invalid_operation() {
	executor.parse_and_validate_options('auto', '', 'invalid', false, '') or {
		assert err.msg().contains('Invalid operation')
		return
	}
	assert false, 'Should have failed for invalid operation'
}

fn test_parse_options_with_filename() {
	opts := executor.parse_and_validate_options('auto', '', 'lint', false, 'test.rs') or {
		assert false, 'Should not fail: ${err}'
		return
	}
	assert opts.file_name == 'test.rs'
}

// ============== Config Path Validation Tests ==============

fn test_parse_options_with_valid_config_path() {
	// Create temp config file
	temp_dir := os.join_path(os.temp_dir(), 'lintrunner_exec_test_${os.getpid()}')
	os.mkdir_all(temp_dir) or { assert false, 'Failed to create temp dir' }
	defer {
		os.rmdir_all(temp_dir) or {}
	}

	config_path := os.join_path(temp_dir, 'Cargo.toml')
	os.write_file(config_path, '[package]\nname = "test"') or {
		assert false, 'Failed to create config'
	}

	opts := executor.parse_and_validate_options('auto', config_path, 'lint', false, '') or {
		assert false, 'Should not fail with valid config: ${err}'
		return
	}

	// Config path should be set
	if path := opts.config_path {
		assert path == config_path
	} else {
		assert false, 'Config path should be set'
	}
}

fn test_parse_options_with_nonexistent_config() {
	executor.parse_and_validate_options('auto', '/nonexistent/path/config.toml', 'lint',
		false, '') or {
		assert err.msg().contains('does not exist')
		return
	}
	assert false, 'Should have failed for nonexistent config'
}

fn test_parse_options_with_unsupported_config_extension() {
	temp_dir := os.join_path(os.temp_dir(), 'lintrunner_ext_test_${os.getpid()}')
	os.mkdir_all(temp_dir) or { assert false, 'Failed to create temp dir' }
	defer {
		os.rmdir_all(temp_dir) or {}
	}

	bad_config := os.join_path(temp_dir, 'config.xyz')
	os.write_file(bad_config, 'content') or { assert false, 'Failed to create file' }

	executor.parse_and_validate_options('auto', bad_config, 'lint', false, '') or {
		assert err.msg().contains('Unsupported')
		return
	}
	assert false, 'Should have failed for unsupported extension'
}

// ============== Color Utils Tests ==============

fn test_get_lang_label_rust() {
	label := executor.get_lang_label('rust')
	assert label == 'Rust'
}

fn test_get_lang_label_go() {
	label := executor.get_lang_label('go')
	assert label == 'Go'
}

fn test_get_lang_label_python() {
	label := executor.get_lang_label('python')
	assert label == 'Python'
}

fn test_get_lang_label_v() {
	label := executor.get_lang_label('v')
	assert label == 'V'
}

fn test_get_lang_label_unknown() {
	label := executor.get_lang_label('unknown')
	assert label == 'UNKNOWN'
}

fn test_format_status_success() {
	status := executor.format_status(true)
	assert status.contains('PASS')
}

fn test_format_status_failure() {
	status := executor.format_status(false)
	assert status.contains('FAIL')
}
