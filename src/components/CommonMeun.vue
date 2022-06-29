<template>
  <div>
    <el-row class="tac">
      <el-col :span="24">
        <!-- <router-link to="/">Go to Home</router-link>
      <router-link to="/settings">Go to Settings</router-link> -->
        <el-menu
          default-active="2"
          class="el-menu-vertical-demo"
          @open="handleOpen"
          @close="handleClose"
          router
          background-color="#545c64"
          text-color="#fff"
          active-text-color="#ffd04b"
        >
         <el-menu-item index="/">
            <i class="el-icon-menu"></i>
            <span slot="title">单个转换</span>
          </el-menu-item>
          <!-- <el-menu-item index="/mulit">
            <i class="el-icon-menu"></i>
            <span slot="title">批量转换</span>
          </el-menu-item>
          <el-menu-item index="3" disabled>
            <i class="el-icon-document"></i>
            <span slot="title">综合(开发中)</span>
          </el-menu-item> -->
          <el-menu-item index="/settings">
            <i class="el-icon-setting"></i>
            <span slot="title">设置</span>
          </el-menu-item>
        </el-menu>
      </el-col>
    </el-row>
  </div>
</template>

<script>
export default {
  name: "CommonMeun",
  methods: {
    handleOpen(key, keyPath) {
      console.log(key, keyPath);
    },
    handleClose(key, keyPath) {
      console.log(key, keyPath);
    },
  },
};
</script>

<style>
.el-menu-vertical-demo{
    height: 100vh;
}
</style>