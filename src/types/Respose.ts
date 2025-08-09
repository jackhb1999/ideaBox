export interface ApiRespose<T> {
    code: number
    msg: string
    data: T | null
}
