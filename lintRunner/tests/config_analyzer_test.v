module tests

import os
import config_analyzer

// ============== Extension Tests ==============

fn test_supported_extensions_contains_v() {
	assert '.v' in config_analyzer.supported_extensions
}

fn test_supported_extensions_contains_rs() {
	assert '.rs' in config_analyzer.supported_extensions
}

fn test_supported_extensions_contains_go() {
	assert '.go' in config_analyzer.supported_extensions
}

fn test_supported_extensions_contains_py() {
	assert '.py' in config_analyzer.supported_extensions
}

fn test_supported_extensions_contains_js() {
	assert '.js' in config_analyzer.supported_extensions
}

fn test_supported_extensions_contains_ts() {
	assert '.ts' in config_analyzer.supported_extensions
}

fn test_supported_extensions_contains_json() {
	assert '.json' in config_analyzer.supported_extensions
}

fn test_supported_extensions_contains_yaml() {
	assert '.yaml' in config_analyzer.supported_extensions
	assert '.yml' in config_analyzer.supported_extensions
}

fn test_supported_extensions_contains_toml() {
	assert '.toml' in config_analyzer.supported_extensions
}

// ============== Config Type Detection Tests with TempDir ==============

fn test_detect_v_module() {
	// Create temp directory
	temp_dir := os.join_path(os.temp_dir(), 'lintrunner_test_${os.getpid()}')
	os.mkdir_all(temp_dir) or { assert false, 'Failed to create temp dir' }
	defer {
		os.rmdir_all(temp_dir) or {}
	}

	// Create v.mod file
	v_mod_path := os.join_path(temp_dir, 'v.mod')
	os.write_file(v_mod_path, 'Module { name: "test" }') or {
		assert false, 'Failed to create v.mod'
	}

	// Test detection
	typ := config_analyzer.get_config_file_typ(v_mod_path) or {
		assert false, 'Failed to detect config type: ${err}'
		return
	}
	assert typ == 'v_language_module'
}

fn test_detect_rust_crate() {
	temp_dir := os.join_path(os.temp_dir(), 'lintrunner_test_rust_${os.getpid()}')
	os.mkdir_all(temp_dir) or { assert false, 'Failed to create temp dir' }
	defer {
		os.rmdir_all(temp_dir) or {}
	}

	cargo_path := os.join_path(temp_dir, 'Cargo.toml')
	os.write_file(cargo_path, '[package]\nname = "test"') or {
		assert false, 'Failed to create Cargo.toml'
	}

	typ := config_analyzer.get_config_file_typ(cargo_path) or {
		assert false, 'Failed to detect: ${err}'
		return
	}
	assert typ == 'rust_crate'
}

fn test_detect_go_module() {
	temp_dir := os.join_path(os.temp_dir(), 'lintrunner_test_go_${os.getpid()}')
	os.mkdir_all(temp_dir) or { assert false, 'Failed to create temp dir' }
	defer {
		os.rmdir_all(temp_dir) or {}
	}

	go_mod_path := os.join_path(temp_dir, 'go.mod')
	os.write_file(go_mod_path, 'module test\ngo 1.21') or {
		assert false, 'Failed to create go.mod'
	}

	typ := config_analyzer.get_config_file_typ(go_mod_path) or {
		assert false, 'Failed to detect: ${err}'
		return
	}
	assert typ == 'go_module'
}

fn test_detect_package_json() {
	temp_dir := os.join_path(os.temp_dir(), 'lintrunner_test_js_${os.getpid()}')
	os.mkdir_all(temp_dir) or { assert false, 'Failed to create temp dir' }
	defer {
		os.rmdir_all(temp_dir) or {}
	}

	pkg_path := os.join_path(temp_dir, 'package.json')
	os.write_file(pkg_path, '{"name": "test"}') or { assert false, 'Failed to create package.json' }

	typ := config_analyzer.get_config_file_typ(pkg_path) or {
		assert false, 'Failed to detect: ${err}'
		return
	}
	assert typ == 'package_config'
}

fn test_detect_tsconfig() {
	temp_dir := os.join_path(os.temp_dir(), 'lintrunner_test_ts_${os.getpid()}')
	os.mkdir_all(temp_dir) or { assert false, 'Failed to create temp dir' }
	defer {
		os.rmdir_all(temp_dir) or {}
	}

	ts_path := os.join_path(temp_dir, 'tsconfig.json')
	os.write_file(ts_path, '{"compilerOptions": {}}') or {
		assert false, 'Failed to create tsconfig.json'
	}

	typ := config_analyzer.get_config_file_typ(ts_path) or {
		assert false, 'Failed to detect: ${err}'
		return
	}
	assert typ == 'typescript_config'
}

fn test_detect_python_project() {
	temp_dir := os.join_path(os.temp_dir(), 'lintrunner_test_py_${os.getpid()}')
	os.mkdir_all(temp_dir) or { assert false, 'Failed to create temp dir' }
	defer {
		os.rmdir_all(temp_dir) or {}
	}

	pyproject_path := os.join_path(temp_dir, 'pyproject.toml')
	os.write_file(pyproject_path, '[project]\nname = "test"') or {
		assert false, 'Failed to create pyproject.toml'
	}

	typ := config_analyzer.get_config_file_typ(pyproject_path) or {
		assert false, 'Failed to detect: ${err}'
		return
	}
	assert typ == 'python_project'
}

// ============== Unsupported Extension Test ==============

fn test_reject_unsupported_extension() {
	temp_dir := os.join_path(os.temp_dir(), 'lintrunner_test_bad_${os.getpid()}')
	os.mkdir_all(temp_dir) or { assert false, 'Failed to create temp dir' }
	defer {
		os.rmdir_all(temp_dir) or {}
	}

	bad_file := os.join_path(temp_dir, 'file.xyz')
	os.write_file(bad_file, 'content') or { assert false, 'Failed to create test file' }

	config_analyzer.get_config_file_typ(bad_file) or {
		// Should fail - this is expected
		assert err.msg().contains('not supported')
		return
	}
	assert false, 'Should have rejected unsupported extension'
}
