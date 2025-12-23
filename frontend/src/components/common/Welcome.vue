<template>
    <div class="hero" :class="{ dark: isDark }" :style="heroStyle">
        <!-- 背景气泡 -->
        <div class="bubbles">
            <div
                v-for="i in 12"
                :key="i"
                class="bubble"
                :style="{
                    '--i': i,
                    '--c': colors[i % 6],
                    '--s': 0.6 + (i % 4) * 0.2,
                    '--x': (i % 3) * 30 - 15,
                    '--y': (i % 4) * 25 - 10,
                }"
            />
        </div>

        <!-- 主标题 -->
        <h1
            class="title"
            :style="{ opacity: titleOpacity, transform: titleTransform }"
        >
            CJC's Blog
        </h1>
    </div>
</template>

<script setup>
import { useThemeVars } from "naive-ui";
import { computed, ref, onMounted, onUnmounted } from "vue";

const tv = useThemeVars();
const scrollY = ref(0);

/* 仅使用 themeVars 提供的 6 个语义色 */
const colors = computed(() => [
    tv.value.primaryColor,
    tv.value.infoColor,
    tv.value.successColor,
    tv.value.warningColor,
    tv.value.errorColor,
    tv.value.primaryColorSuppl,
]);

/* 判断是否为暗黑模式 */
const isDark = computed(() => {
    return tv.value?.baseColor && tv.value.baseColor !== "#ffffff";
});

/* 将主题颜色和背景色传递给样式 */
const heroStyle = computed(() => ({
    "--primary-color": tv.value?.primaryColor || "#000",
    "--bg-color": tv.value?.baseColor || "#fff",
}));

/* 标题的透明度和位置随滚动而变化 */
const titleOpacity = computed(() => {
    return Math.max(0, 1 - scrollY.value / 500);
});

const titleTransform = computed(() => {
    const translateY = Math.min(scrollY.value / 3, 200);
    return `translateY(-${translateY}px)`;
});

const handleScroll = () => {
    scrollY.value = window.scrollY;
};

onMounted(() => {
    window.addEventListener("scroll", handleScroll);
});

onUnmounted(() => {
    window.removeEventListener("scroll", handleScroll);
});
</script>

<style scoped>
.hero {
    position: relative;
    height: 100vh;
    display: flex;
    align-items: center;
    justify-content: center;
    overflow: hidden;
    background-color: var(--bg-color);
}

/* 背景气泡 */
.bubbles {
    position: absolute;
    inset: 0;
}

.bubble {
    position: absolute;
    width: 160px;
    height: 160px;
    border-radius: 36px;
    background: var(--c);
    opacity: 0.12;
    filter: blur(20px);
    transform: scale(var(--s))
        translate(calc(var(--x) * 1vw), calc(var(--y) * 1vh));
    animation: float 12s ease-in-out infinite;
    animation-delay: calc(var(--i) * 0.8s);
    transition: opacity 0.3s ease, filter 0.3s ease;
}

/* 暗黑模式下提升气泡的视觉效果 */
.hero.dark .bubble,
:global([data-n-theme="dark"]) .bubble {
    opacity: 0.28;
    filter: blur(18px) brightness(1.1) saturate(1.2);
}

@keyframes float {
    0%,
    100% {
        transform: scale(var(--s))
            translate(calc(var(--x) * 1vw), calc(var(--y) * 1vh));
    }
    50% {
        transform: scale(calc(var(--s) * 1.2))
            translate(calc((var(--x) + 10) * 1vw), calc((var(--y) - 8) * 1vh));
    }
}

/* 主标题 */
.title {
    font-size: clamp(3rem, 10vw, 7rem);
    font-weight: 800;
    letter-spacing: 0.05em;
    color: var(--primary-color);
    text-shadow: 0 4px 24px var(--primary-color), 0 0 1px #fff;
    animation: breathe 4s ease-in-out infinite;
}

@keyframes breathe {
    0%,
    100% {
        transform: translateY(0) scale(1);
        opacity: 0.95;
    }
    50% {
        transform: translateY(-8px) scale(1.02);
        opacity: 1;
    }
}
</style>
