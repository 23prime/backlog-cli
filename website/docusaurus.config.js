// @ts-check

const version = process.env.DOCS_VERSION || 'dev';

/** @type {import('@docusaurus/types').Config} */
const config = {
  // title: 'backlog-cli',
  title: 'Backlog CLI',
  tagline: "An unofficial CLI tool for Nulab's Backlog",
  favicon: 'img/favicon.ico',

  url: 'https://23prime.github.io',
  baseUrl: '/backlog-cli/',

  organizationName: '23prime',
  projectName: 'backlog-cli',

  onBrokenLinks: 'throw',
  onBrokenMarkdownLinks: 'warn',

  i18n: {
    defaultLocale: 'en',
    locales: ['en', 'ja'],
    localeConfigs: {
      en: { label: 'English' },
      ja: { label: '日本語' },
    },
  },

  markdown: {
    mermaid: true,
  },

  themes: ['@docusaurus/theme-mermaid'],

  presets: [
    [
      'classic',
      /** @type {import('@docusaurus/preset-classic').Options} */
      ({
        docs: {
          routeBasePath: '/',
          sidebarPath: './sidebars.js',
        },
        blog: false,
        theme: {
          customCss: './src/css/custom.css',
        },
      }),
    ],
  ],

  themeConfig:
    /** @type {import('@docusaurus/preset-classic').ThemeConfig} */
    ({
      navbar: {
        title: 'backlog-cli',
        items: [
          {
            type: 'localeDropdown',
            position: 'right',
          },
          {
            type: 'html',
            position: 'right',
            value: `<span>${version}</span>`,
          },
          {
            href: 'https://github.com/23prime/backlog-cli',
            label: 'GitHub',
            position: 'right',
          },
        ],
      },
      footer: {
        style: 'dark',
        links: [
          {
            label: 'GitHub',
            href: 'https://github.com/23prime/backlog-cli',
          },
        ],
        copyright: `Copyright © ${new Date().getFullYear()} 23prime. Built with Docusaurus.`,
      },
    }),
};

module.exports = config;
