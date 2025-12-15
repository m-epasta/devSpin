'use client';

import { useState, useEffect } from 'react';
import '../styles/LandingStyle.scss';

interface FeatureCardProps {
  icon: string;
  title: string;
  description: string;
  accentColor: 'terminal' | 'cyan' | 'magenta';
}

const FeatureCard: React.FC<FeatureCardProps> = ({ icon, title, description, accentColor }) => (
  <div className={`feature-card glow-${accentColor}`}>
    <div className="feature-icon">{icon}</div>
    <h3>{title}</h3>
    <p>{description}</p>
  </div>
);
// TODO: implement cli command remove and changes
const ProgressBar: React.FC = () => {
  const [progress, setProgress] = useState(0);

  useEffect(() => {
    const interval = setInterval(() => {
      setProgress(prev => {
        if (prev >= 100) {
          clearInterval(interval);
          return 100;
        }
        return prev + 2;
      });
    }, 50);

    return () => clearInterval(interval);
  }, []);

  return (
    <div className="progress-bar">
      <div className="progress-fill" style={{ width: `${progress}%` }}></div>
      <span className="progress-text">Setting up devspin.yml...</span>
    </div>
  );
};

  const TerminalDemo: React.FC = () => {
    const [visibleText, setVisibleText] = useState('');
    const [showOutput, setShowOutput] = useState(false);

    const command = 'devspin init';

    const output = (
      <div className="folder-structure">
        <div className="folder-item">📄 devspin.yml created with sample configuration</div>
        <div className="folder-item indent secondary">📦 PostgreSQL database service included</div>
        <div className="folder-item indent secondary">📦 Redis cache service included</div>
        <div className="folder-item indent secondary">🛠️ Linting, formatting, and testing enabled</div>
        <div className="folder-item success">✓ devspin.yml initialized successfully!</div>
      </div>
    );

    const handleCopy = () => {
      navigator.clipboard.writeText('devspin init');
    };

    const renderHighlightedCommand = (text: string) => {
      if (!text) return null;
      if (text.startsWith('devspin init')) {
        return (
          <>
            <span className="terminal-command-main">devspin</span>{' '}
            <span className="terminal-subcommand">init</span>
          </>
        );
      }
      return <span>{text}</span>;
    };

    useEffect(() => {
      const typeText = () => {
        let i = 0;
        const interval = setInterval(() => {
          if (i <= command.length) {
            setVisibleText(command.slice(0, i));
            i++;
          } else {
            clearInterval(interval);
            setTimeout(() => setShowOutput(true), 500);
          }
        }, 100);
      };

      const timer = setTimeout(typeText, 1000);
      return () => clearTimeout(timer);
    }, []);

    const showCursor = visibleText.length < command.length;

    return (
      <>
        <div className="terminal-header">
          <div className="terminal-header-left">
            <div className="terminal-dots">
              <span className="dot red"></span>
              <span className="dot yellow"></span>
              <span className="dot green"></span>
            </div>
            <span className="terminal-title">devspin</span>
          </div>
          <button className="copy-icon-button" onClick={handleCopy}>
            <img src="/file.svg" alt="Copy" width="16" height="16" />
          </button>
        </div>
        <div className="terminal-body">
          <div className="terminal-line input-line">
            <span className="terminal-prompt">$</span>
            {visibleText.length > 0 && (
              <>
                <span className="terminal-command">
                  {renderHighlightedCommand(visibleText)}
                </span>
                {showCursor && <span className="terminal-cursor">|</span>}
              </>
            )}
          </div>
          {showOutput && (
            <div className="terminal-demo-output">
              {output}
            </div>
          )}
        </div>
      </>
    );
  };

interface TerminalLineProps {
  command?: string;
  output?: string[];
  delay: number;
  type?: 'input' | 'output';
}



const Landing: React.FC = () => {
  return (
    <div className="landing-page">
      {/* Hero Section */}
      <section className="hero-section">
        <div className="hero-content">
          <h1 className="text-gradient">Manage your dev environment in a single config file</h1>
          <p className="hero-subtitle">Automate dependencies, services, CI/CD, and environment variables with devspin.yml</p>

          <div className="terminal-container glow-terminal">
            <TerminalDemo />
          </div>
        </div>
      </section>

      {/* Features Section */}
      <section className="features-section">
        <div className="features-content">
          <div className="features-grid">
            <FeatureCard
              icon="�️"
              title="Built-in Developer Tools"
              description="Linting, formatting, and testing automation with a single command."
              accentColor="terminal"
            />
            <FeatureCard
              icon="🐳"
              title="Docker Integration"
              description="Seamless containerization for portable development environments."
              accentColor="cyan"
            />
            <FeatureCard
              icon="⚙️"
              title="Service Orchestration"
              description="Run databases, caches, and services alongside your application."
              accentColor="magenta"
            />
          </div>
        </div>
      </section>

      {/* Stats Section */}
      <section className="stats-section">
        <div className="stats-content">
          <div className="stat-item">
            <div className="stat-number glow-terminal">50+</div>
            <div className="stat-label">Framework Templates</div>
          </div>
          <div className="stat-item">
            <div className="stat-number glow-cyan">10k+</div>
            <div className="stat-label">Developers Using</div>
          </div>
          <div className="stat-item">
            <div className="stat-number glow-magenta">99.9%</div>
            <div className="stat-label">Uptime Reliability</div>
          </div>
        </div>
      </section>

      {/* Integration Section */}
      <section className="integrations-section">
        <h2>Supported Frameworks & Tools</h2>
        <div className="integration-logos">
          <div className="logo-item">Next.js</div>
          <div className="logo-item">React</div>
          <div className="logo-item">Tailwind</div>
          <div className="logo-item">Node.js</div>
          <div className="logo-item">Docker</div>
        </div>
      </section>

      {/* CTA Section */}
      <section className="cta-section">
        <h2>Ready to simplify your dev workflow?</h2>
        <button className="cta-button glow-terminal">Initialize devSpin</button>
        <p className="cta-description">Start with devspin init - free and open source.</p>
      </section>
    </div>
  );
};

export default Landing;
