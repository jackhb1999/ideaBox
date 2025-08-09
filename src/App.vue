<script setup lang="ts">
import {Reactive, reactive, ref} from "vue";
import {invoke} from "@tauri-apps/api/core";
import {SettingOutlined} from '@ant-design/icons-vue';
import {message} from 'ant-design-vue';

// const greetMsg = ref("");
// const name = ref("");

// const color = ref("");
//
// async function changeColor(colorVal: string) {
//   color.value = colorVal;
// }

import {useWindowSize} from '@vueuse/core'
import {CardStatus, Color, Item, newItem} from "@/types/Item.ts";
import {ApiRespose} from "@/types/Respose.ts";

const {width} = useWindowSize()

// const showNotification = () => {
//   notification.info({
//     message: `功能还未实现`,
//     placement: 'topLeft',
//   });
// };

const showMessage = () => {
  message.info('功能还未实现!');
};

// 列表
const listData: Reactive<Item[]> = reactive([])

// 新增
async function addItem() {
  const NewItem = new newItem();
  console.log(88, listData.indexOf(NewItem.addItem))
  if (listData.indexOf(NewItem.addItem) === -1) {
    listData.unshift(NewItem.addItem)
  }
}

// 修改
async function updateItem(item: Item, index: number) {
  item.status = CardStatus.view
  item.updateTime = new Date()
  if (item.id == null) {
    invoke<ApiRespose<Item>>("create", {params: item}).then((res: ApiRespose<Item>) => {
      if (res.code == 200) {
        item = res.data as Item
      }
    })
  } else {
    invoke<ApiRespose<Item>>("update", {params: item}).then((res: ApiRespose<Item>) => {
      if (res.code == 200) {
        item = res.data as Item
      }
    })
  }
  listData[index] = item
}

// 删除
async function deleteItem(item: Item) {
  const index = listData.indexOf(item);
  if (index !== -1) {
    if (item.id != null) {
      invoke<ApiRespose<null>>("delete", {id: item.id}).then((res: ApiRespose<null>) => {
        if (res.code == 200) {
          message.success(res.msg)
        }
      })
    }
    listData.splice(index, 1); // 移除第一个
  }
}

const foldStatus = ref(false);

// 展开
async function foldOn() {
  listData.forEach((item: Item) => {
    if (item.status !== CardStatus.edit) {
      item.status = CardStatus.view
    }
  })
  foldStatus.value = false;
}

// 收起
async function foldOff() {
  listData.forEach((item: Item) => {
    if (item.status !== CardStatus.edit) {
      item.status = CardStatus.fold
    }
  })
  foldStatus.value = true;
}

async function getList() {
  const list: Item[] = await invoke("list", {});
  console.log(131, list)
  listData.push(...list)
}

getList()
// async function greet() {
//   // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
//   greetMsg.value = await invoke("greet", {name: name.value});
// }
</script>

<template>
  <main class="space-y-1">

    <div class="justify-items-end">
      <!--      最上bar-->
      <div class="bg-gradient-to-bl from-dark-50 via-gray-700 to-dark-50 bg-blend-color-dodge
      backdrop-blur-sm flex basis-full h-2em w-12em justify-between items-center">
        <div class="inline-block backdrop-blur-sm bg-dark-700 select-none">
          <span>1</span>
        </div>
        <div class="inline-block text-zinc-50">
          <SettingOutlined @click="showMessage"/>
        </div>
        <div class="inline-block text-zinc-50">
          <ColumnHeightOutlined v-if="foldStatus" @click="foldOn"/>

          <VerticalAlignMiddleOutlined v-else="foldStatus" @click="foldOff"/>
        </div>
        <div class="inline-block text-zinc-50">
          <PlusCircleOutlined @click="addItem"/>
        </div>
        <div class="inline-block mr-4 text-zinc-50">
          <ArrowRightOutlined @click="showMessage"/>
        </div>
      </div>
    </div>
    <template v-for="(item,index) in listData">
      <div class="justify-items-end" v-if="item.status === CardStatus.edit">
        <!--      新增卡片-->
        <a-card hoverable class="container" :style="{width,backgroundColor:item.color}">
          <template #cover>
            <a-flex gap="middle" vertical>
              <a-row>
                <a-col :span="4">
                  <a-divider>
                    <a-button type="dashed" shape="circle" ghost block @click="item.color = Color.blue">
                      <template #icon>
                        <BuildTwoTone :two-tone-color="Color.blue"/>
                      </template>
                    </a-button>
                  </a-divider>
                </a-col>
                <a-col :span="4">
                  <a-divider>
                    <a-button type="dashed" shape="circle" ghost block @click="item.color = Color.red">
                      <template #icon>
                        <ExclamationCircleTwoTone :two-tone-color="Color.red"/>
                      </template>
                    </a-button>
                  </a-divider>
                </a-col>
                <a-col :span="4">
                  <a-divider>
                    <a-button type="dashed" shape="circle" ghost block @click="item.color = Color.orange">
                      <template #icon>
                        <CarryOutTwoTone :two-tone-color="Color.orange"/>
                      </template>
                    </a-button>
                  </a-divider>
                </a-col>
                <a-col :span="4">
                  <a-divider>
                    <a-button type="dashed" shape="circle" ghost block @click="item.color  = Color.lime">
                      <template #icon>
                        <MessageTwoTone :two-tone-color="Color.lime"/>
                      </template>
                    </a-button>
                  </a-divider>
                </a-col>
                <a-col :span="4">
                  <a-divider>
                    <a-button type="dashed" shape="circle" ghost block @click="item.color = Color.rose">
                      <template #icon>
                        <PushpinTwoTone :two-tone-color="Color.rose"/>
                      </template>
                    </a-button>
                  </a-divider>
                </a-col>
                <a-col :span="4">
                  <a-divider>
                    <a-button type="dashed" shape="circle" ghost block @click="item.color  = Color.purple">
                      <template #icon>
                        <BulbTwoTone :two-tone-color="Color.purple"/>
                      </template>
                    </a-button>
                  </a-divider>
                </a-col>
              </a-row>
            </a-flex>
          </template>
          <template #actions>
            <CheckOutlined @click="updateItem(item,index)"/>
            <CloseOutlined @click="deleteItem(item)"/>
          </template>
          <a-card-meta>
            <template #description>
              <a-textarea
                  v-model:value="item.content"
                  placeholder=""
                  auto-size
              />
            </template>
          </a-card-meta>
        </a-card>
      </div>
      <div class="justify-items-end" v-else-if="item.status === CardStatus.view">
        <!--      查看卡片-->
        <a-card hoverable class="container" :style="{width,backgroundColor:item.color}">
          <a-card-meta>
            <template #description>
              <div class="font-sans text-base font-normal hyphens-auto  leading-snug break-words
             text-white" @click="item.status++">
                {{ item.content }}
              </div>
            </template>
          </a-card-meta>
          <template #actions>
            <DeleteOutlined @click="deleteItem(item)"/>
            <ShareAltOutlined @click="showMessage"/>
            <CoffeeOutlined @click="showMessage"/>
          </template>
        </a-card>
      </div>
      <div class="justify-items-end w-auto"
           v-else-if="item.status === CardStatus.fold && (width / item.content.length <20 )">
        <!--      折叠卡片-->
        <a-card hoverable class="container outline-white" :style="{width,backgroundColor:item.color}">
          <a-card-meta>
            <template #description>
              <div class="font-sans text-base font-normal hyphens-auto tracking-wider leading-snug
            truncate text-white" @click="item.status++">
                {{ item.content }}
              </div>
            </template>
          </a-card-meta>
        </a-card>
      </div>
      <div class="justify-items-end w-auto" v-else>
        <a-card hoverable class="outline-white" :style="{backgroundColor:item.color}">
          <a-card-meta>
            <template #description>
              <div class="font-sans text-base font-normal hyphens-auto tracking-wider leading-snug
            truncate text-white" @click="item.status++"> {{ item.content }}
              </div>
            </template>
          </a-card-meta>
        </a-card>
      </div>
    </template>
  </main>
</template>

<style scoped>
::v-deep(.ant-card-body) {
  padding: 10px;
}
</style>