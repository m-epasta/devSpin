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
              <div className="diagram-box">Spin File</div>
              <div className="diagram-arrow">↓</div>
            </div>
            <div className="diagram-step">
              <div className="diagram-box">Target Interpreting</div>
              <div className="diagram-arrow">↓</div>
            </div>
            <div className="diagram-step">
              <div className="diagram-box">Target Execution</div>
            </div>
            <div className="diagram-branches">
              <div className="diagram-branch">
                <div className="diagram-arrow">→</div>
                <div className="diagram-box small">Container</div>
              </div>
              <div className="diagram-branch">
                <div className="diagram-arrow">→</div>
                <div className="diagram-box small">Cluster</div>
              </div>
              <div className="diagram-branch">
                <div className="diagram-arrow">→</div>
                <div className="diagram-box small">Deployment</div>
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
