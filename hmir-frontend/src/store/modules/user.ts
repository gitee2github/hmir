import { defineStore } from 'pinia';
import { invoke } from "@tauri-apps/api/tauri";
import ElMessage from '@/utils/message';
import { sessionStorage } from '@/utils/sessionStorage'

interface LoginData {
  host: string;
  port: number;
  username: string;
  password: string;
}

interface logoutData {
  host: string;
}

// 第一个参数是应用程序中 store 的唯一 id
export const useUsersStore = defineStore('user', {
  //存储用户数据
  state: () => {
    //获取登录成功后保存在本地的数据
    const user = sessionStorage.get('user')
    return {
      host: user.host || '',
      port: user.port || 0,
      username: user.username || '',
    };
  },
  //计算属性
  getters: {
  },
  //异步同步函数
  actions: {
    //登录
    cmdLogin(loginData: LoginData) {
      return new Promise<void>((resolve, reject) => {
        invoke("cmd_login", { ...loginData })
          .then(response => {
            if (response) {
              this.host = loginData.host;
              this.port = loginData.port;
              this.username = loginData.username;
              //将清除password后的数据保存到会话存储中，防止刷新页面后丢失数据
              const { password, ...value } = loginData;
              sessionStorage.set('user', value)
              ElMessage.success('登录成功');
              resolve();
            } else {
              ElMessage({
                message: '登录失败，请重试',
                type: 'error',
                customClass: 'login-message-error',
              });
              reject(response);
            }
          })
          .catch(error => {
            reject(error);
          });
      })
    },
    //注销
    cmdlogout(logoutData: logoutData) {
      return new Promise<void>((resolve, reject) => {
        invoke("cmd_logout", { ...logoutData })
          .then(response => {
            //判断是否注销成功
            if (!response) {
              //重置所有的用户信息
              this.$reset()
              sessionStorage.remove('user')
              ElMessage.success('注销成功');
            } else {
              ElMessage.error({
                message: '注销失败,请联系管理员',
                customClass: 'login-message-error'
              });
            }
            resolve();
          })
          .catch(error => {
            reject(error);
          });
      })
    }
  }
});
