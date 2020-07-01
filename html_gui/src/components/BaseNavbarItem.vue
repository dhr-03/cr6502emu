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
            >
                {{ route.name }}
            </a>

            <a v-else>
                {{ route.name }}
            </a>
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

<style scoped>
    .cr-nav-item {
        text-transform: capitalize;
    }
</style>
