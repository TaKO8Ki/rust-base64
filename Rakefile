# frozen_string_literal: true

require "bundler/gem_tasks"
require "rake/extensiontask"

task build: :compile

Rake::ExtensionTask.new("rust_base64") do |ext|
  ext.lib_dir = "lib/rust_base64"
end

task default: %i[clobber compile]
