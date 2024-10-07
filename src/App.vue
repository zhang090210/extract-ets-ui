<script setup>
// When using the Tauri API npm package:
import {invoke} from '@tauri-apps/api/core';
import {ref, onMounted} from 'vue';
import {save, message} from '@tauri-apps/plugin-dialog';

let papers = ref([]);
let get_papers = () => {
  invoke('get_papers').then((res) => {
    papers.value = []
    res.forEach((p) => {
      p.paper_time = p.paper_time.join('-');
      papers.value.push(p)
    })
  }).catch(async (err) => {
    await message(err, {title: 'extract-ets(v5.7.1)', kind: 'error'});
  })
};

let export_pdf = async (sourcePath) => {
  const pdfPath = await save({
    filters: [
      {
        name: 'Portable Document Format',
        extensions: ['pdf'],
      },
    ],
  });
  console.log(pdfPath);
  invoke('export_pdf', {sourcePath: sourcePath, pdfPath: pdfPath}).then(async (res) => {
    await message("export successful!", {title: 'extract-ets(v5.7.1)', kind: 'info'});
  }).catch(async (err) => {
    await message(err, {title: 'extract-ets(v5.7.1)', kind: 'error'});
  })
}

onMounted(() => {
  get_papers();
})
</script>

<template>
  <h1 class="title" onselectstart='return false'>
    e听说答案提取(高中版)
    <span>v5.7.1</span>
  </h1>
  <div class="papers_list" onselectstart='return false'>
    <div style="display: flex">
      <v-btn style="max-height: 30px; margin: auto 0; max-width: 10vw" @click="get_papers">刷新</v-btn>
      <h4 style="margin-left: 10px; margin-top: 5px">题库列表：</h4>
    </div>
    <div style="margin-top: 5px" v-for="paper in papers" :key="paper.id">
      <v-card>
        <v-card-item>
          <v-card-title>题目id: {{ paper.paper_id }}</v-card-title>
          <v-card-subtitle>题目路径: {{ paper.paper_path }}</v-card-subtitle>
          <v-card-subtitle>修改时间: {{ paper.paper_time }}</v-card-subtitle>
        </v-card-item>
        <v-card-actions>
          <v-btn @click="export_pdf(paper.paper_path)">导出PDF</v-btn>
        </v-card-actions>
      </v-card>
    </div>
  </div>
</template>

<style scoped>
.title {
  text-align: center;
  font-size: 30px;
  margin: 20px 0;
}

.title span {
  font-size: 20px;
  color: #999;
}

.papers_list {
  width: 100%;
  height: 85%;
}
</style>
