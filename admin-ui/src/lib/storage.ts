const API_KEY_STORAGE_KEY = 'adminApiKey'

// 使用 sessionStorage 而非 localStorage，降低凭据长期残留风险
// 关闭浏览器标签页后需要重新输入 API Key
export const storage = {
  getApiKey: () => sessionStorage.getItem(API_KEY_STORAGE_KEY),
  setApiKey: (key: string) => sessionStorage.setItem(API_KEY_STORAGE_KEY, key),
  removeApiKey: () => sessionStorage.removeItem(API_KEY_STORAGE_KEY),
}
