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
import {CardStatus, Color, Item, newItem} from "../src/types/Item.ts";

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
const listData: Reactive<Item[]> = reactive([
  {
    color: Color.blue,
    content: '',
    createTime: new Date(),
    updateTime: undefined,
    status: CardStatus.edit,
    order: 0
  },
  {
    color: Color.blue,
    content: '我思故我在 在一在二在三在四 在五在六在其再把在救灾是',
    createTime: new Date(),
    updateTime: undefined,
    status: CardStatus.view,
    order: 0
  },
  {
    color: Color.red,
    content: '我思故我在',
    createTime: new Date(),
    updateTime: undefined,
    status: CardStatus.fold,
    order: 0
  },
  {
    color: Color.orange,
    content: '我思故我在 在一在二在三在四 在五在六在其再把在救灾是',
    createTime: new Date(),
    updateTime: undefined,
    status: CardStatus.fold,
    order: 0
  },
  {
    color: Color.lime,
    content: '我思故我在',
    createTime: new Date(),
    updateTime: undefined,
    status: CardStatus.fold,
    order: 0
  },
  {
    color: Color.rose,
    content: '我思故我在',
    createTime: new Date(),
    updateTime: undefined,
    status: CardStatus.fold,
    order: 0
  },

])

// 新增
async function addItem() {
  const NewItem = new newItem();
  console.log(88, listData.indexOf(NewItem.addItem))
  if (listData.indexOf(NewItem.addItem) === -1) {
    listData.unshift(NewItem.addItem)
  }
}

// 修改
async function updateItem(item: Item) {
  item.status = CardStatus.view
}

// 删除
async function deleteItem(item: Item) {
  const index = listData.indexOf(item);
  if (index !== -1) {
    listData.splice(index, 1); // 移除第一个 2
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
  const list:Item[] = await invoke("list", {});
  console.log(131,list)
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
    <template v-for="item in listData">
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
            <CheckOutlined @click="updateItem(item)"/>
            <CloseOutlined/>
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