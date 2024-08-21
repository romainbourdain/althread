import type * as Preset from "@docusaurus/preset-classic";
import type { Config } from "@docusaurus/types";
import { themes as prismThemes } from "prism-react-renderer";

const config: Config = {
  title: "Althread",
  tagline: "Documentation officielle du langage de programmation Althread",
  favicon: "img/favicon.ico",

  url: "https://romainbourdain.github.io",
  baseUrl: "/althread/",

  organizationName: "romainbourdain",
  projectName: "althread",
  deploymentBranch: "main",

  onBrokenLinks: "throw",
  onBrokenMarkdownLinks: "warn",

  i18n: {
    defaultLocale: "fr",
    locales: ["fr"],
  },

  presets: [
    [
      "classic",
      {
        docs: {
          sidebarPath: "./sidebars.ts",
          editUrl: "https://github.com/romainbourdain/althread/tree/main/",
        },
        theme: {
          customCss: "./src/css/custom.css",
        },
      } satisfies Preset.Options,
    ],
  ],

  themeConfig: {
    navbar: {
      title: "Althread",
      logo: {
        alt: "Althread Logo",
        src: "img/logo.svg",
      },
      items: [
        {
          type: "docSidebar",
          sidebarId: "guideSidebar",
          position: "left",
          label: "Guide",
        },
        {
          type: "docSidebar",
          sidebarId: "apiSidebar",
          position: "left",
          label: "Références",
        },
        {
          type: "docSidebar",
          sidebarId: "exampleSidebar",
          position: "left",
          label: "Exemples",
        },
        {
          href: "https://github.com/facebook/docusaurus",
          label: "GitHub",
          position: "right",
        },
        {
          type: "localeDropdown",
          position: "right",
        },
      ],
    },
    footer: {
      style: "dark",
      links: [
        {
          title: "Documentation",
          items: [
            {
              label: "(Guide)",
              to: "/",
            },
            {
              label: "(Références)",
              to: "/",
            },
            {
              label: "(Exemples)",
              to: "/",
            },
          ],
        },

        {
          title: "Plus",
          items: [
            {
              label: "GitHub",
              href: "https://github.com/romainbourdain/althread/",
            },
          ],
        },
      ],
      copyright: `Copyright © ${new Date().getFullYear()} Romain Bourdain.`,
    },
    prism: {
      theme: prismThemes.github,
      darkTheme: prismThemes.dracula,
    },
  } satisfies Preset.ThemeConfig,
};

export default config;
