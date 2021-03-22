// Copyright 2021 Kodai Matsumoto

#pragma once

#include <vector>
#include <vulkan/vulkan.hpp>

/** GLFW が必要としているインスタンス拡張を一括取得する */
std::vector<const char *> getRequiredInstanceExtensions() noexcept;

/** Vulkan インスタンスを生成する */
vk::UniqueHandle<vk::Instance, vk::DispatchLoaderStatic> createInstance(
    const std::vector<const char *> &instance_exts, const std::vector<const char *> &layers);
