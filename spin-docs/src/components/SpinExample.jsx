// src/components/SpinExample.jsx
import React from 'react';
import CodeBlock from '@theme/CodeBlock';
import './SpinExample.css';

export default function SpinExample({ title, code, terminal, description }) {
  return (
    <div className="spin-example">
      {title && <h3>{title}</h3>}
      {description && <p className="spin-example-description">{description}</p>}
      <div className="example-grid">
        <div className="code-block">
          <CodeBlock language="spin">{code}</CodeBlock>
        </div>
        {terminal && (
          <div className="terminal-block">
            <div className="terminal-header">
              <div className="terminal-dots">
                <div className="terminal-dot red"></div>
                <div className="terminal-dot yellow"></div>
                <div className="terminal-dot green"></div>
              </div>
              <span>Terminal</span>
            </div>
            <CodeBlock language="bash">{terminal}</CodeBlock>
          </div>
        )}
      </div>
    </div>
  );
}
