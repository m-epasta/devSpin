// src/prism-languages/prism-spin.js
if (typeof Prism !== 'undefined') {
  Prism.languages.spin = {
    'comment': {
      pattern: /;.*/,
      greedy: true
    },
    'shebang': {
      pattern: /^#! .*/,
      greedy: true
    },
    'keyword': {
      pattern: /\b(?:app|run|needs|type|port|mode|env|workspace|build|scaling|health|version|name|description|workspace)\b/,
      lookbehind: true
    },
    'builtin': {
      pattern: /\b(?:postgres|redis|rabbitmq|mongodb|mysql|mariadb|elasticsearch|minio|vault|consul)\b/
    },
    'function': {
      pattern: /\b(?:validate-config|optimize-image|health-check|exec-scripts)\b/
    },
    'string': {
      pattern: /"[^"]*"/,
      greedy: true
    },
    'number': /\b\d+\b/,
    'punctuation': /[{}[\],:]/,
    'operator': /\|/,
    'variable': {
      pattern: /\$\{[^}]+\}/,
      greedy: true
    },
    'symbol': {
      pattern: /@\w+|@\[[^\]]+\]|#[\w-]+|<[^>]+>/,
      greedy: true
    }
  };
}
