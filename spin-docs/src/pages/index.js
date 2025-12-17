import React from 'react';
import Link from '@docusaurus/Link';
import useDocusaurusContext from '@docusaurus/useDocusaurusContext';
import Layout from '@theme/Layout';
import SpinExample from '@site/src/components/SpinExample';
import Steps from '@site/src/components/Steps';
import './index.css';

export default function Home() {
  const { siteConfig } = useDocusaurusContext();

  return (
    <Layout
      title={siteConfig.title}
      description="Declare once. Run anywhere. The multi-target application manifest language.">
      <div className="homepage">
        {/* Hero Section */}
        <div className="hero">
          <div className="container">
            <h1 className="hero__title">{siteConfig.title}</h1>
            <p className="hero__subtitle">{siteConfig.tagline}</p>
            <p className="hero__description">
              Stop writing the same configurations in different languages. Write your app manifest once,
              and let SPIN generate Dockerfiles, Kubernetes YAML, Terraform configs, and more.
            </p>
            <div className="hero__buttons">
              <Link
                className="button button--primary button--lg"
                to="/docs/quickstart">
                Get Started
              </Link>
              <Link
                className="button button--secondary button--lg"
                to="https://github.com/spin">
                GitHub
              </Link>
            </div>
          </div>
        </div>

        {/* Code Example Section */}
        <div className="code-showcase">
          <div className="container">
            <div className="section-header">
              <h2>One File → Multiple Platforms</h2>
              <p>Write your application manifest once, deploy everywhere</p>
            </div>
            <SpinExample
              title="Simple Web App Example"
              code={`#! spn 1.0
{ name: "my-webapp" }

app "web" {
  type: "node"
  port: 3000
  needs: [postgres, redis]
}`}
              terminal={`$ spin run
🚀 Starting my-webapp...
📦 Generated Dockerfile
🐳 Generated docker-compose.yml
☸️  Generated k8s manifests
☁️  Generated Terraform configs
✅ App running at http://localhost:3000`}
              description="SPIN detects your stack, sets up databases, and generates all necessary configuration files automatically."
            />
          </div>
        </div>

        {/* Features Section */}
        <div className="features">
          <div className="container">
            <div className="section-header">
              <h2>Why Developers Choose SPIN</h2>
              <p>Solve deployment complexity once and for all</p>
            </div>
            <div className="row">
              <div className="col col--4">
                <div className="feature">
                  <div className="feature-icon">🔄</div>
                  <h3>Multi-Platform Deployment</h3>
                  <p>One SPN file works everywhere - Docker, Kubernetes, AWS, and more. No more maintaining separate configs for each platform.</p>
                </div>
              </div>
              <div className="col col--4">
                <div className="feature">
                  <div className="feature-icon">🧠</div>
                  <h3>Smart Dependency Detection</h3>
                  <p>SPIN automatically detects and configures your service dependencies. Databases, caches, and services are set up automatically.</p>
                </div>
              </div>
              <div className="col col--4">
                <div className="feature">
                  <div className="feature-icon">⚙️</div>
                  <h3>Environment Management</h3>
                  <p>Smart defaults with environment-specific overrides. Development, staging, and production configs from a single source.</p>
                </div>
              </div>
            </div>
            <div className="row features-row-2">
              <div className="col col--4">
                <div className="feature">
                  <div className="feature-icon">🏃‍♂️</div>
                  <h3>Local Development</h3>
                  <p>Run your full stack locally with hot-reload. No more complex setup scripts or conflicting services.</p>
                </div>
              </div>
              <div className="col col--4">
                <div className="feature">
                  <div className="feature-icon">📈</div>
                  <h3>Developer Productivity</h3>
                  <p>Focus on your code, not infrastructure. SPIN handles the complexity so you can build faster.</p>
                </div>
              </div>
              <div className="col col--4">
                <div className="feature">
                  <div className="feature-icon">🔒</div>
                  <h3>Production Ready</h3>
                  <p>Generated configurations follow best practices and security standards for each platform.</p>
                </div>
              </div>
            </div>
          </div>
        </div>

        {/* Get Started Section */}
        <div className="get-started">
          <div className="container">
            <div className="section-header">
              <h2>Get Started in 60 Seconds</h2>
              <p>Ready to try SPIN? It takes less than a minute to get your first app running.</p>
            </div>
            <div className="get-started-content">
              <div className="steps-section">
                <Steps>
                  <div>
                    <h4>Install SPIN CLI</h4>
                    <p>Download and install the SPIN command-line tool.</p>
                    <code>curl -fsSL https://spin.dev/install.sh | sh</code>
                  </div>
                  <div>
                    <h4>Create your app manifest</h4>
                    <p>Initialize a new SPIN project with a basic manifest.</p>
                    <code>spin init my-app && cd my-app</code>
                  </div>
                  <div>
                    <h4>Run it</h4>
                    <p>SPIN detects your stack and starts your application.</p>
                    <code>spin run</code>
                  </div>
                </Steps>
              </div>
              <div className="cta-section">
                <Link
                  className="button button--primary button--lg"
                  to="/docs/quickstart">
                  Read the Quick Start Guide →
                </Link>
              </div>
            </div>
          </div>
        </div>

        {/* Testimonials Section */}
        <div className="testimonials">
          <div className="container">
            <div className="section-header">
              <h2>Loved by Developers</h2>
              <p>Join thousands of developers simplifying their deployments</p>
            </div>
            <div className="row">
              <div className="col col--6">
                <div className="testimonial">
                  <blockquote>
                    "SPIN reduced our deployment complexity by 80%. One manifest file instead of 15 different configs."
                  </blockquote>
                  <cite>— Senior DevOps Engineer</cite>
                </div>
              </div>
              <div className="col col--6">
                <div className="testimonial">
                  <blockquote>
                    "Finally, a tool that understands my app needs. No more wrestling with YAML."
                  </blockquote>
                  <cite>— Full Stack Developer</cite>
                </div>
              </div>
            </div>
          </div>
        </div>

        {/* Community Section */}
        <div className="community">
          <div className="container">
            <div className="section-header">
              <h2>Join the Community</h2>
              <p>SPIN is open source and community-driven</p>
            </div>
            <div className="community-links">
              <Link
                className="community-link"
                to="https://github.com/spin">
                <div className="community-icon">🐙</div>
                <span>GitHub</span>
              </Link>
              <Link
                className="community-link"
                to="https://discord.gg/spin">
                <div className="community-icon">💬</div>
                <span>Discord</span>
              </Link>
              <Link
                className="community-link"
                to="https://twitter.com/spin_dev">
                <div className="community-icon">🐦</div>
                <span>Twitter</span>
              </Link>
            </div>
          </div>
        </div>
      </div>
    </Layout>
  );
}
