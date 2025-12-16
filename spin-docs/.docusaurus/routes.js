import React from 'react';
import ComponentCreator from '@docusaurus/ComponentCreator';

export default [
  {
    path: '/search',
    component: ComponentCreator('/search', 'c5a'),
    exact: true
  },
  {
    path: '/docs',
    component: ComponentCreator('/docs', '334'),
    routes: [
      {
        path: '/docs',
        component: ComponentCreator('/docs', '92a'),
        routes: [
          {
            path: '/docs',
            component: ComponentCreator('/docs', '4a5'),
            routes: [
              {
                path: '/docs/',
                component: ComponentCreator('/docs/', 'a8c'),
                exact: true,
                sidebar: "tutorialSidebar"
              },
              {
                path: '/docs/concepts',
                component: ComponentCreator('/docs/concepts', 'fb5'),
                exact: true,
                sidebar: "tutorialSidebar"
              },
              {
                path: '/docs/faq',
                component: ComponentCreator('/docs/faq', 'e79'),
                exact: true,
                sidebar: "tutorialSidebar"
              },
              {
                path: '/docs/first-project',
                component: ComponentCreator('/docs/first-project', '451'),
                exact: true,
                sidebar: "tutorialSidebar"
              },
              {
                path: '/docs/quickstart',
                component: ComponentCreator('/docs/quickstart', 'e30'),
                exact: true,
                sidebar: "tutorialSidebar"
              }
            ]
          }
        ]
      }
    ]
  },
  {
    path: '*',
    component: ComponentCreator('*'),
  },
];
