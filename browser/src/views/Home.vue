<template>
  <v-card>
    <v-toolbar color="#F9FAFD" shadow-none>
      <v-toolbar-title>device name</v-toolbar-title>

      <v-spacer></v-spacer>

      <v-btn icon>
        <v-icon>mdi-magnify</v-icon>
      </v-btn>

      <v-btn icon>
        <v-icon>mdi-upload</v-icon>
      </v-btn>


      <v-btn icon>
        <v-icon>mdi-share</v-icon>
      </v-btn>

      <template v-slot:extension>
        <v-tabs
            v-model="tab"
            align-tabs="title"
        >
          <v-tab
              v-for="item in items"
              :key="item"
              :value="item"
          >
            {{ item }}
          </v-tab>
        </v-tabs>
      </template>
    </v-toolbar>

    <v-window v-model="tab">
      <v-window-item
          v-for="item in items"
          :key="item"
          :value="item"
      >
        <v-card flat>
          <v-card-text v-text="text"></v-card-text>
        </v-card>
      </v-window-item>
    </v-window>
  </v-card>
</template>
<script lang="ts" setup>
const tab  = null;
const items = ['Images', "Videos", "Audio", "Documents"]
</script>
