module.exports = {
  title: 'SPIN',
  tagline: 'Declare once. Run anywhere.',
  url: 'https://spin.dev',
  baseUrl: '/',
  onBrokenLinks: 'warn',
  onBrokenMarkdownLinks: 'warn',
  organizationName: 'spin',
  projectName: 'docs',

  presets: [
    [
      '@docusaurus/preset-classic',
      {
        docs: {
          sidebarPath: require.resolve('./sidebars.js'),
          editUrl: 'https://github.com/spin/docs/edit/main/',
          showLastUpdateAuthor: true,
          showLastUpdateTime: true,
        },
        theme: {
          customCss: require.resolve('./src/css/custom.css'),
        },
      },
    ],
  ],



  themeConfig: {
    navbar: {
      title: 'SPIN',
      items: [
        {
          type: 'docSidebar',
          sidebarId: 'tutorialSidebar',
          position: 'left',
          label: 'Docs',
        },
        {
          to: '/docs/quickstart',
          label: 'Quick Start',
          position: 'left',
        },
        {
          href: 'https://github.com/spin',
          label: 'GitHub',
          position: 'right',
        },
      ],
    },
    footer: {
      style: 'dark',
      links: [
        {
          title: 'Docs',
          items: [
            {
              label: 'Quick Start',
              to: '/docs/quickstart',
            },
            {
              label: 'Concepts',
              to: '/docs/concepts',
            },
            {
              label: 'First Project',
              to: '/docs/first-project',
            },
            {
              label: 'FAQ',
              to: '/docs/faq',
            },
          ],
        },
        {
          title: 'Community',
          items: [
            {
              label: 'GitHub',
              href: 'https://github.com/spin',
            },
            {
              label: 'Discord',
              href: 'https://discord.gg/spin',
            },
            {
              label: 'Twitter',
              href: 'https://twitter.com/spin_dev',
            },
          ],
        },
        {
          title: 'Project',
          items: [
            {
              label: 'FAQ',
              to: '/docs/faq',
            },
            {
              label: 'GitHub',
              href: 'https://github.com/spin',
            },
          ],
        },
      ],
      copyright: `Copyright © ${new Date().getFullYear()} SPIN. Built with Docusaurus.`,
    },
    colorMode: {
      defaultMode: 'dark',
      disableSwitch: false,
      respectPrefersColorScheme: true,
    },
    prism: {
      theme: require('prism-react-renderer').themes.dracula,
      additionalLanguages: ['bash', 'yaml', 'docker'],
    },
    algolia: {
      appId: 'YOUR_APP_ID',
      apiKey: 'YOUR_SEARCH_API_KEY',
      indexName: 'spin',
      contextualSearch: true,
    },
  },
};
