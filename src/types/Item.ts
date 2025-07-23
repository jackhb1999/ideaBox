export interface Item {
    // 颜色
    color: Color
    // 内容
    content: string
    // 创建时间
    createTime: Date
    // 修改时间
    updateTime?: Date
    // 状态
    status: CardStatus

    // 顺序
    order: number
}

export class newItem {
    addItem: Item = {
        color: Color.blue,
        content: '',
        createTime: new Date(),
        updateTime: undefined,
        status: CardStatus.edit,
        order: 0
    }
}


export enum Color {
    blue = '#4d7ce4',
    red = '#dc655f',
    orange = '#f5b402',
    lime = '#7bc75b',
    rose = '#f2b9b2',
    purple = '#a65fd7',
}

export enum CardStatus {
    // 折叠
    fold,
    // 查看
    view,
    // 编辑
    edit ,
}
