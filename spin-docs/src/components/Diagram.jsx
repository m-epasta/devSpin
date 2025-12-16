// src/components/Diagram.jsx
import React from 'react';
import './Diagram.css';

export default function Diagram({ type, title }) {
  const getDiagramContent = (type) => {
    switch (type) {
      case 'targets':
        return (
          <div className="diagram-targets">
            <div className="diagram-step">
              <div className="diagram-box">SPN File</div>
              <div className="diagram-arrow">↓</div>
            </div>
            <div className="diagram-step">
              <div className="diagram-box">Target Processor</div>
              <div className="diagram-arrow">↓</div>
            </div>
            <div className="diagram-step">
              <div className="diagram-box">Platform Configs</div>
            </div>
            <div className="diagram-branches">
              <div className="diagram-branch">
                <div className="diagram-arrow">→</div>
                <div className="diagram-box small">Dockerfile</div>
              </div>
              <div className="diagram-branch">
                <div className="diagram-arrow">→</div>
                <div className="diagram-box small">K8s YAML</div>
              </div>
              <div className="diagram-branch">
                <div className="diagram-arrow">→</div>
                <div className="diagram-box small">Terraform</div>
              </div>
            </div>
          </div>
        );
      default:
        return <div className="diagram-placeholder">Diagram: {type}</div>;
    }
  };

  return (
    <div className="diagram-container">
      {title && <h4 className="diagram-title">{title}</h4>}
      {getDiagramContent(type)}
    </div>
  );
}
