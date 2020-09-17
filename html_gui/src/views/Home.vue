<template>
    <div class="crg-container crg-mg-top">
        <h2>Open Local Project</h2>

        <hr>

        <div class="crl-saved-projects-container">
            <div
                class="uk-grid uk-margin-remove"

                v-for="(prj, index) in projectsList"
                :key="index"
            >
                <div class="crl-project uk-width">
                    <div class="crl-project-name  uk-width-1-3 uk-inline uk-text-truncate">
                        <span>{{ prj.meta.name }}</span>
                    </div>

                    <div class="uk-width-1-3 uk-inline uk-text-center">
                        {{ getTimeAgo(prj.meta.lastMod) }} ago
                    </div>

                    <router-link
                        :to="{name: 'Project', params: {pid: prj.meta.pid}}"

                        class="crl-project-go uk-width-1-3 uk-inline uk-text-right"
                    >
                        <font-awesome-icon icon="arrow-circle-right"/>
                    </router-link>

                </div>
            </div>

            <div
                v-if="!projectsListByDate.length"

                class="uk-text-center"
            >
                <span>No saved projects found.</span>
            </div>
        </div>

        <hr>

        <div class="uk-text-center">
            <a
                @click="addProjectAndGo"
                class="crl-project-add"
            >
                <font-awesome-icon icon="plus-square"/>
                New Project
            </a>
        </div>
    </div>
</template>

<script>
    import Tools from "../assets/js/tools";
    import {mapGetters, mapActions} from "vuex";

    export default {
        name: "Home",

        computed: {
            ...mapGetters("prj", {
                    "projectsList": "getAllProjects"
                }
            ),

            projectsListByDate() {
                return this.projectsList.sort((a, b) => b.meta.lastMod - a.meta.lastMod);
            }
        },

        methods: {
            ...mapActions("prj", [
                "createNewProject"
            ]),

            getTimeAgo(date) {
                return Tools.timeSince(date);
            },

            async addProjectAndGo() {
                let newPrjId = await this.createNewProject();

                await this.$router.push({
                    name: 'Project',

                    params: {
                        pid: newPrjId
                    }
                });
            }
        }
    }
</script>

<style lang="less" scoped>
    @import "../../node_modules/open-color/open-color";

    .crl-saved-projects-container {
        max-height: 50vh;

        overflow-y: auto;
    }

    .crl-project {
        border: 1px solid #bbb;

        margin: 0.5em;
        padding: 0.5em;
    }

    .crl-project-name {
        font-weight: bold;
        color: #fff;
    }

    .crl-project-go {
        padding-right: 0.5em;

        color: @oc-blue-3;
    }

    .crl-project-add {
        color: @oc-yellow-5;
    }
</style>
