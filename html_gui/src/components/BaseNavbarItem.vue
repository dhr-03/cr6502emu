<template>
    <router-link
        :to="targetObj"
        v-slot="{ href, route, navigate, isActive, isExactActive}"
    >
        <li
            class="cr-nav-item"
            :class="{'uk-active': (isActive && !matchExact) || isExactActive}"
        >
            <a
                v-if="clickable"
                :href="href"
                @click="navigate"

                v-t="'router.' + route.name"
            />

            <a
                v-else
                v-t="'router.' + route.name"
            />
        </li>
    </router-link>
</template>

<script>
    export default {
        name: "BaseNavbarItem",
        props: {
            target: {
                type: [String, Object]
            },

            matchExact: {
                type: Boolean,
                default: false
            },

            clickable: {
                type: Boolean,
                default: true
            }
        },

        computed: {
            targetObj() {
                if (typeof this.target === "string") {
                    return {
                        name: this.target
                    };

                } else {
                    return this.target;
                }
            }
        }
    }
</script>

<style lang="less" scoped>
    .cr-nav-item {
        text-transform: capitalize;

        a {
            min-height: 60px;
        }
    }
</style>
