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

  const TerminalDemo: React.FC = () => {
    const [visibleText, setVisibleText] = useState('');
    const [showOutput, setShowOutput] = useState(false);
    const [currentCommandIndex, setCurrentCommandIndex] = useState(0);

    const commands = [
      'devspin init',
      'devspin config',
      'devspin run',
      'devspin shell'
    ];

    const commandOutputs = {
      'devspin init': (
        <div className="folder-structure">
          <div className="folder-item">📄 devspin.yml created with sample configuration</div>
          <div className="folder-item indent secondary">📦 PostgreSQL database service included</div>
          <div className="folder-item indent secondary">📦 Redis cache service included</div>
          <div className="folder-item indent secondary">🛠️ Linting, formatting, and testing enabled</div>
          <div className="folder-item success">✓ devspin.yml initialized successfully!</div>
        </div>
      ),
      'devspin config': (
        <div className="folder-structure">
          <div className="folder-item">🔍 Analyzing project structure...</div>
          <div className="folder-item indent secondary">📦 Detected Node.js project</div>
          <div className="folder-item indent secondary">📦 Found package.json dependencies</div>
          <div className="folder-item indent secondary">⚙️ Configuring services and environment</div>
          <div className="folder-item success">✓ Configuration updated successfully!</div>
        </div>
      ),
      'devspin run': (
        <div className="folder-structure">
          <div className="folder-item">🚀 Starting development environment...</div>
          <div className="folder-item indent secondary">🐳 Starting PostgreSQL container</div>
          <div className="folder-item indent secondary">🐳 Starting Redis container</div>
          <div className="folder-item indent secondary">📱 Starting application server</div>
          <div className="folder-item success">✓ Development environment running!</div>
        </div>
      ),
      'devspin shell': (
        <div className="folder-structure">
          <div className="folder-item">🔧 Opening devspin shell...</div>
          <div className="folder-item indent secondary">📊 Services status: All running</div>
          <div className="folder-item indent secondary">🛠️ Tools: Linter, formatter, tester ready</div>
          <div className="folder-item indent secondary">💻 Interactive mode enabled</div>
          <div className="folder-item success">✓ Shell ready for commands!</div>
        </div>
      )
    };

    const currentCommand = commands[currentCommandIndex];
    const output = commandOutputs[currentCommand as keyof typeof commandOutputs];

    const handleCopy = () => {
      navigator.clipboard.writeText(currentCommand);
    };

    const renderHighlightedCommand = (text: string) => {
      if (!text) return null;
      const parts = text.split(' ');
      if (parts.length >= 2 && parts[0] === 'devspin') {
        return (
          <>
            <span className="terminal-command-main">{parts[0]}</span>{' '}
            <span className="terminal-subcommand">{parts.slice(1).join(' ')}</span>
          </>
        );
      }
      return <span>{text}</span>;
    };

    useEffect(() => {
      const typeText = () => {
        let i = 0;
        const interval = setInterval(() => {
          if (i <= currentCommand.length) {
            setVisibleText(currentCommand.slice(0, i));
            i++;
          } else {
            clearInterval(interval);
            setTimeout(() => {
              setShowOutput(true);
              // Move to next command after showing output
              setTimeout(() => {
                setShowOutput(false);
                setVisibleText('');
                setCurrentCommandIndex((prev) => (prev + 1) % commands.length);
              }, 2000); // Show output for 2 seconds
            }, 500);
          }
        }, 100);
      };

      const timer = setTimeout(typeText, 1000);
      return () => clearTimeout(timer);
    }, [currentCommandIndex, currentCommand, commands.length]);

    const showCursor = visibleText.length < currentCommand.length;

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
              icon="🛠️"
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
              description="Configure and run databases, caches, and services alongside your application."
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
            <div className="stat-label">Templates</div>
          </div>
          <div className="stat-item">
            <div className="stat-number glow-cyan">10k+</div>
            <div className="stat-label">Developers Using</div>
          </div>
          <div className="stat-item">
            <div className="stat-number glow-magenta">24/24</div>
            <div className="stat-label">Disponibility</div>
          </div>
        </div>
      </section>

      {/* Integration Section */}
      <section className="integrations-section">
        <h2>Supported Frameworks & Tools</h2>
        <div className="integration-logos">
          <div className="logo-item">Node.js</div>
          <div className="logo-item">Rust</div>
          <div className="logo-item">Go</div>
          <div className="logo-item">Kubernetes</div>
          <div className="logo-item">Docker</div>
        </div>
      </section>

      {/* CTA Section */}
      <section className="cta-section">
        <h2>Ready to simplify your dev workflow?</h2>
        <button className="cta-button glow-terminal">Start devSpin</button>
        <p className="cta-description">Start with devspin - free and open source.</p>
      </section>

      {/* Footer Section */}
      <footer className="footer-section">
        <div className="footer-content">
          <div className="footer-brand">
            <h3 className="text-gradient">devSpin</h3>
            <p>Development Environment Manager</p>
          </div>
          <div className="footer-social">
            <h4>Connect with us</h4>
            <div className="social-links">
              {/* TODO: add svg icons */}
              <a href="https://github.com/your-repo" className="social-link glow-terminal" target="_blank" rel="noopener noreferrer">
                <span className="social-icon">🐙</span>
                <span className="social-text">GitHub</span>
              </a>
              <a href="https://twitter.com/your-handle" className="social-link glow-cyan" target="_blank" rel="noopener noreferrer">
                <span className="social-icon">🐦</span>
                <span className="social-text">Twitter</span>
              </a>
              <a href="https://discord.gg/your-invite" className="social-link glow-magenta" target="_blank" rel="noopener noreferrer">
                <span className="social-icon">💬</span>
                <span className="social-text">Discord</span>
              </a>
              <a href="https://t.me/your-channel" className="social-link glow-terminal" target="_blank" rel="noopener noreferrer">
                <span className="social-icon">✈️</span>
                <span className="social-text">Telegram</span>
              </a>
            </div>
          </div>
        </div>
        <div className="footer-bottom">
          <p>&copy; 2025 devSpin. Free and open source.</p>
        </div>
      </footer>
    </div>
  );
};

export default Landing;
