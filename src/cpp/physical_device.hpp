// Copyright 2021 Kodai Matsumoto

#pragma once

#include <iostream>
#include <vulkan/vulkan.hpp>

/** 物理デバイスの種別を表した文字列を取得する */
std::string physicalDeviceTypeToStr(vk::PhysicalDeviceType device_type) noexcept;

/** 物理デバイスを一覧表示する */
void dumpPhysicalDevices(const std::vector<vk::PhysicalDevice> &devices) noexcept;

/** 使用する物理デバイスとキューファミリを決定する */
void selectPhysicalDeviceAndQueueFamily(const vk::Instance &instance,
                                        const std::vector<vk::PhysicalDevice> &devices,
                                        vk::PhysicalDevice *selected_device,
                                        uint32_t *queue_family_index, uint32_t *queue_count);

/** 物理デバイスが対応しているデバイス拡張を一覧表示する */
void dumpDeviceExtensions(const vk::PhysicalDevice &physical_device);
