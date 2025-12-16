// src/components/Steps.jsx
import React from 'react';
import './Steps.css';

export default function Steps({ children }) {
  const childrenArray = React.Children.toArray(children);

  return (
    <div className="steps-container">
      {childrenArray.map((child, index) => (
        <div key={index} className="step-item">
          <div className="step-number">
            {index + 1}
          </div>
          <div className="step-content">
            {child}
          </div>
        </div>
      ))}
    </div>
  );
}
