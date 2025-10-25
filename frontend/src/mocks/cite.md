# Naive UI 布局四件套超细笔记

（`Layout` / `Grid` / `Flex` / `Space`）

* * *

## 1️⃣ n-layout —— 页面的「骨架」

### 📌 作用

- 负责 **整站** 或 **整页** 的大布局：Header / Sider / Content / Footer
- 自带 **折叠、响应式、暗黑模式** 主题

### 📌 组成

- `n-layout` 根容器
- `n-layout-header` 顶部栏
- `n-layout-sider` 侧边栏（可折叠）
- `n-layout-content` 内容区
- `n-layout-footer` 底部栏

### 📌 常用 API

| 属性  | 说明  | 示例  |
| --- | --- | --- |
| `has-sider` | 是否包含侧边栏 | `<n-layout has-sider>` |
| `position="absolute"` | 绝对定位 | 弹窗遮罩层 |
| `sider-placement="right"` | 侧边栏在右边 | 右抽屉 |
| `collapsed` / `collapsed-width` | 折叠控制 | 后台菜单 |

### 📌 最小示例

```vue
<n-layout has-sider>
  <n-layout-sider
    collapse-mode="width"
    :collapsed-width="64"
    :width="240"
    show-trigger
  >
    <SideMenu />
  </n-layout-sider>

  <n-layout>
    <n-layout-header bordered>
      <NavBar />
    </n-layout-header>

    <n-layout-content content-style="padding: 24px;">
      <router-view />
    </n-layout-content>
  </n-layout>
</n-layout>
```

* * *

## 2️⃣ n-grid —— 12 栅格系统

### 📌 作用

- **区块级** 响应式栅格，类似 Bootstrap
- 支持 **断点、自动换行、偏移、对齐**

### 📌 组成

- `n-grid` 容器
- `n-gi`（grid item）子项

### 📌 常用 API

| 属性  | 说明  | 示例  |
| --- | --- | --- |
| `cols` | 每行列数 | `:cols="3"` |
| `x-gap` / `y-gap` | 水平/垂直间距 | `:x-gap="16"` |
| `responsive` | 断点对象 | `:responsive="{ xs: 1, sm: 2, md: 3 }"` |
| `offset` | 左侧空几格 | `offset="1"` |

### 📌 最小示例 —— 响应式卡片

```vue
<n-grid cols="1 s:2 m:3 l:4" :x-gap="16" :y-gap="16">
  <n-gi v-for="item in list" :key="item.id">
    <n-card>{{ item.name }}</n-card>
  </n-gi>
</n-grid>
```

* * *

## 3️⃣ n-flex —— Flexbox 一行对齐胶水

### 📌 作用

- **行内级** 一行多列快速对齐
- 等价于手写 `display: flex` + `justify-content` + `align-items`

### 📌 常用 API

| 属性  | 说明  | 示例  |
| --- | --- | --- |
| `justify` | 主轴对齐 | `justify="center"` |
| `align` | 交叉轴对齐 | `align="flex-start"` |
| `wrap` | 是否换行 | `wrap` |
| `inline` | 行内 flex | `inline` |

### 📌 最小示例

```vue
<n-flex justify="space-between" align="center">
  <span>标题</span>
  <n-button>操作</n-button>
</n-flex>
```

* * *

## 4️⃣ n-space —— 纯间距管家

### 📌 作用

- **原子级** 控制 **元素与元素之间的空隙**
- 支持 **垂直/水平/换行/对齐**

### 📌 常用 API

| 属性  | 说明  | 示例  |
| --- | --- | --- |
| `size` | 间距大小 | `size="small"` 或 `:size="[8, 16]"` |
| `vertical` | 垂直方向 | `vertical` |
| `inline` | 行内元素 | `inline` |
| `wrap` | 自动换行 | `wrap` |

### 📌 最小示例

```vue
<n-space vertical :size="12">
  <n-input v-model:value="title" placeholder="标题" />
  <n-input v-model:value="desc" placeholder="描述" />
</n-space>
```

* * *

## 🎯 选型流程图（口诀）

```text
页面整体 → n-layout
  ├── 大模块 → n-grid
  │     ├── 对齐 → n-flex
  │     └── 空隙 → n-space
```

* * *

## 📑 速查表

| 场景  | 推荐组件 |
| --- | --- |
| 整站/整页框架 | `n-layout` |
| 卡片列表/仪表盘 | `n-grid` |
| 一行按钮左右对齐 | `n-flex` |
| 表单项垂直间距 | `n-space` |

* * *

> **总结一句话**：  
> 按“骨架 → 区块 → 对齐 → 间距”四步，用 Layout 搭楼，Grid 分房，Flex 摆家具，Space 留过道——**90% 场景不用写 CSS**。

* * *

## 关于n-flex和n-space替换问题

结论先行  
官方确实建议：

> “**能用 `n-flex` 就不要用 `n-space`**”。

原因是：

1.  **能力覆盖**：`n-flex` 通过 CSS `gap` 就能完成 `n-space` 的“间距”职责；
2.  **渲染层差异**：`n-space` 会在每个子节点外包一层 `div`，可能带来选择器、事件冒泡、SSR 等问题；
3.  **兼容性**：`gap` 在 IE 下不可用，现代浏览器已全部支持（Naive UI 2.37+ 起默认开启）。

* * *

### 如何用 `n-flex` 完全替代 `n-space` 做“元素与元素之间的空隙”？

`n-flex` 提供了与 `n-space` 一一对应的 `size` 属性，**写法几乎 1:1 替换**：

| 原 Space 写法 | 等价 Flex 写法 |
| --- | --- |
| `<n-space size="12">` | `<n-flex size="12">` |
| `<n-space vertical>` | `<n-flex vertical>` |
| `<n-space wrap>` | `<n-flex wrap>` |
| `<n-space justify="space-between">` | `<n-flex justify="space-between">` |

示例：

```vue
<!-- 原来用 n-space -->
<n-space size="medium" wrap>
  <n-button>A</n-button>
  <n-button>B</n-button>
</n-space>

<!-- 等价 n-flex -->
<n-flex size="medium" wrap>
  <n-button>A</n-button>
  <n-button>B</n-button>
</n-flex>
```

* * *

### 如果默认值不满意，如何自定义 gap？

1.  **组件级**
    
    ```vue
    <n-flex :style="{ gap: '8px 12px' }"> … </n-flex>
    ```
    
2.  **主题级**（全局统一）
    
    ```ts
    const themeOverrides = {
      Flex: {
        gapMedium: '8px 12px'
      }
    }
    ```
    
    ```vue
    <n-config-provider :theme-overrides="themeOverrides">
      <App />
    </n-config-provider>
    ```
    

* * *

### 小结

- ✅ **新项目** → 直接 `n-flex`；
- ⚠️ **需要兼容 IE11** → 仍用 `n-space`；
- 🔄 **老代码迁移** → 把 `n-space` 换 `n-flex`，API 几乎一致，零成本。