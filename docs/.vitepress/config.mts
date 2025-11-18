import { defineConfig } from 'vitepress'

export default defineConfig({
  title: 'uvup',
  description: 'Blazingly Fast Python Environment Manager',

  lastUpdated: true,
  cleanUrls: true,

  base: '/uvup/',

  head: [
    ['link', { rel: 'icon', type: 'image/svg+xml', href: '/uvup/logo.svg' }],
    ['meta', { name: 'theme-color', content: '#5f67ee' }],
    ['meta', { property: 'og:type', content: 'website' }],
    ['meta', { property: 'og:locale', content: 'en' }],
    ['meta', { property: 'og:title', content: 'uvup | Python Environment Manager' }],
    ['meta', { property: 'og:site_name', content: 'uvup' }],
  ],

  locales: {
    root: {
      label: 'English',
      lang: 'en',
      themeConfig: {
        sidebar: [
          {
            text: 'Getting Started',
            items: [
              { text: 'Installation', link: '/guide/installation' },
              { text: 'Quick Start', link: '/guide/quick-start' },
              { text: 'Core Concepts', link: '/guide/core-concepts' },
            ]
          },
          {
            text: 'Command Reference',
            items: [
              { text: 'Overview', link: '/commands/' },
              { text: 'Environment', link: '/commands/environment' },
              { text: 'Project', link: '/commands/project' },
              { text: 'Package', link: '/commands/package' },
              { text: 'Shell', link: '/commands/shell' },
            ]
          },
          {
            text: 'Use Cases',
            items: [
              { text: 'Overview', link: '/use-cases/' },
              { text: 'Workflows', link: '/use-cases/workflows' },
              { text: 'Advanced', link: '/use-cases/advanced' },
            ]
          }
        ],
        editLink: {
          pattern: 'https://github.com/KercyDing/uvup/edit/main/docs/:path',
          text: 'Edit this page on GitHub'
        },
        footer: {
          message: 'Released under the MIT License.',
          copyright: 'Copyright © 2024-present uvup contributors'
        }
      }
    },
    zh: {
      label: '简体中文',
      lang: 'zh-CN',
      link: '/zh/',
      themeConfig: {
        sidebar: [
          {
            text: '快速开始',
            items: [
              { text: '安装', link: '/zh/guide/installation' },
              { text: '快速入门', link: '/zh/guide/quick-start' },
              { text: '核心概念', link: '/zh/guide/core-concepts' },
            ]
          },
          {
            text: '命令参考',
            items: [
              { text: '概述', link: '/zh/commands/' },
              { text: '虚拟环境', link: '/zh/commands/environment' },
              { text: '项目管理', link: '/zh/commands/project' },
              { text: '包管理', link: '/zh/commands/package' },
              { text: 'Shell 集成', link: '/zh/commands/shell' },
            ]
          },
          {
            text: '使用案例',
            items: [
              { text: '概述', link: '/zh/use-cases/' },
              { text: '日常工作流', link: '/zh/use-cases/workflows' },
              { text: '高级用法', link: '/zh/use-cases/advanced' },
            ]
          }
        ],
        editLink: {
          pattern: 'https://github.com/KercyDing/uvup/edit/main/docs/:path',
          text: '在 GitHub 上编辑此页'
        },
        footer: {
          message: '基于 MIT 许可发布',
          copyright: 'Copyright © 2024-present uvup 贡献者'
        },
        docFooter: {
          prev: '上一页',
          next: '下一页'
        },
        outline: {
          label: '页面导航'
        },
        lastUpdated: {
          text: '最后更新于',
          formatOptions: {
            dateStyle: 'short',
            timeStyle: 'medium'
          }
        },
        langMenuLabel: '多语言',
        returnToTopLabel: '回到顶部',
        sidebarMenuLabel: '菜单',
        darkModeSwitchLabel: '主题',
        lightModeSwitchTitle: '切换到浅色模式',
        darkModeSwitchTitle: '切换到深色模式'
      }
    }
  },

  themeConfig: {
    logo: '/logo.svg',

    socialLinks: [
      { icon: 'github', link: 'https://github.com/KercyDing/uvup' }
    ],

    search: {
      provider: 'local'
    }
  }
})
