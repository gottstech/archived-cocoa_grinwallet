#
# Be sure to run `pod lib lint cocoa_grinwallet.podspec' to ensure this is a
# valid spec before submitting.
#
# Any lines starting with a # are optional, but their use is encouraged
# To learn more about a Podspec see https://guides.cocoapods.org/syntax/podspec.html
#

Pod::Spec.new do |s|
  s.name             = 'cocoa_grinwallet'
  s.version          = '1.0.1'
  s.summary          = 'Grin Wallet IOS Libs. With embedded Grin Relay service.'

# This description is used to generate tags and improve search results.
#   * Think: What does it do? Why did you write it? What is the focus?
#   * Try to keep it short, snappy and to the point.
#   * Write the description between the DESC delimiters below.
#   * Finally, don't worry about the indent, CocoaPods strips it!

  s.description      = <<-DESC
TODO: Add long description of the pod here.
                       DESC

  s.homepage         = 'https://github.com/gottstech/grin-wallet/wiki'
  # s.screenshots     = 'www.example.com/screenshots_1', 'www.example.com/screenshots_2'
  s.license          = { :type => 'Apache License, Version 2.0', :file => 'LICENSE' }
  s.author           = { 'Gary Yu' => 'gairy.yu@gmail.com' }
  s.source           = { :git => 'https://github.com/gottstech/cocoa_grinwallet.git', :tag => 'v' +  s.version.to_s }
  # s.social_media_url = 'https://twitter.com/<TWITTER_USERNAME>'

  s.swift_version = '5.0'
  s.ios.deployment_target = '8.0'

  s.source_files = 'cocoa_grinwallet/Classes/**/*'
  s.vendored_libraries = 'cocoa_grinwallet/Library/*.a'
  
  # s.resource_bundles = {
  #   'cocoa_grinwallet' => ['cocoa_grinwallet/Assets/*.png']
  # }

  # s.public_header_files = 'Pod/Classes/**/*.h'
  # s.frameworks = 'UIKit', 'MapKit'
  # s.dependency 'AFNetworking', '~> 2.3'
end
