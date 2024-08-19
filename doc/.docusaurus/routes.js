import React from 'react';
import ComponentCreator from '@docusaurus/ComponentCreator';

export default [
  {
    path: '/__docusaurus/debug',
    component: ComponentCreator('/__docusaurus/debug', '5ff'),
    exact: true
  },
  {
    path: '/__docusaurus/debug/config',
    component: ComponentCreator('/__docusaurus/debug/config', '5ba'),
    exact: true
  },
  {
    path: '/__docusaurus/debug/content',
    component: ComponentCreator('/__docusaurus/debug/content', 'a2b'),
    exact: true
  },
  {
    path: '/__docusaurus/debug/globalData',
    component: ComponentCreator('/__docusaurus/debug/globalData', 'c3c'),
    exact: true
  },
  {
    path: '/__docusaurus/debug/metadata',
    component: ComponentCreator('/__docusaurus/debug/metadata', '156'),
    exact: true
  },
  {
    path: '/__docusaurus/debug/registry',
    component: ComponentCreator('/__docusaurus/debug/registry', '88c'),
    exact: true
  },
  {
    path: '/__docusaurus/debug/routes',
    component: ComponentCreator('/__docusaurus/debug/routes', '000'),
    exact: true
  },
  {
    path: '/docs',
    component: ComponentCreator('/docs', 'a76'),
    routes: [
      {
        path: '/docs',
        component: ComponentCreator('/docs', '467'),
        routes: [
          {
            path: '/docs',
            component: ComponentCreator('/docs', 'd9b'),
            routes: [
              {
                path: '/docs/api/intro',
                component: ComponentCreator('/docs/api/intro', '0a0'),
                exact: true,
                sidebar: "apiSidebar"
              },
              {
                path: '/docs/example/',
                component: ComponentCreator('/docs/example/', 'b63'),
                exact: true,
                sidebar: "exampleSidebar"
              },
              {
                path: '/docs/guide/channels/read',
                component: ComponentCreator('/docs/guide/channels/read', '249'),
                exact: true,
                sidebar: "guideSidebar"
              },
              {
                path: '/docs/guide/channels/write',
                component: ComponentCreator('/docs/guide/channels/write', 'da6'),
                exact: true,
                sidebar: "guideSidebar"
              },
              {
                path: '/docs/guide/debugging',
                component: ComponentCreator('/docs/guide/debugging', 'ddb'),
                exact: true,
                sidebar: "guideSidebar"
              },
              {
                path: '/docs/guide/getting-started/hello-world',
                component: ComponentCreator('/docs/guide/getting-started/hello-world', 'ceb'),
                exact: true,
                sidebar: "guideSidebar"
              },
              {
                path: '/docs/guide/getting-started/installation',
                component: ComponentCreator('/docs/guide/getting-started/installation', 'e4e'),
                exact: true,
                sidebar: "guideSidebar"
              },
              {
                path: '/docs/guide/intro',
                component: ComponentCreator('/docs/guide/intro', '0b6'),
                exact: true,
                sidebar: "guideSidebar"
              },
              {
                path: '/docs/guide/process/arguments',
                component: ComponentCreator('/docs/guide/process/arguments', '761'),
                exact: true,
                sidebar: "guideSidebar"
              },
              {
                path: '/docs/guide/process/global',
                component: ComponentCreator('/docs/guide/process/global', 'ea8'),
                exact: true,
                sidebar: "guideSidebar"
              },
              {
                path: '/docs/guide/process/simple-process',
                component: ComponentCreator('/docs/guide/process/simple-process', 'df8'),
                exact: true,
                sidebar: "guideSidebar"
              },
              {
                path: '/docs/guide/test',
                component: ComponentCreator('/docs/guide/test', '8d9'),
                exact: true,
                sidebar: "guideSidebar"
              }
            ]
          }
        ]
      }
    ]
  },
  {
    path: '/',
    component: ComponentCreator('/', 'e5f'),
    exact: true
  },
  {
    path: '*',
    component: ComponentCreator('*'),
  },
];
