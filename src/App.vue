<script setup>
// When using the Tauri API npm package:
import {invoke} from '@tauri-apps/api/core';
import {ref, onMounted} from 'vue';
import {save, message} from '@tauri-apps/plugin-dialog';

let papers = ref([]);
let paper_types = [ // `v-select` 的选项数组
    { title: '高中普通试题', value: 'SeniorCommonPaper' },
  ]
let get_papers = () => {
  invoke('get_papers').then((res) => {
    papers.value = []
    res.forEach((p) => {
      p.paper_time = p.paper_time.join('-');
      p.paper_type = null;
      papers.value.push(p)
    })
  }).catch(async (err) => {
    await message(err, {title: 'extract-ets(v5.7.1)', kind: 'error'});
  })
};

let get_select = (paper) => {
  console.log(paper);
}

let view_result = async (sourcePath, paperType) => {
  // console.log(pdfPath);
  invoke('view_result', {paperPath: sourcePath, paperType: paperType}).then(async (res) => {
    console.log(res)
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
          <v-select
              label="选择题目类型"
              v-model="paper.paper_type"
              :items=paper_types
              variant="outlined"
              max-width="45%"
              @update:modelValue="get_select"
          ></v-select>
          <v-btn @click="view_result(paper.paper_path, paper.paper_type)">查看答案</v-btn>
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
