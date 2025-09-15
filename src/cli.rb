#!/usr/bin/env ruby

require 'thor'
require 'json'
require_relative 'commands/add'
require_relative 'commands/list'
require_relative 'commands/complete'
require_relative 'commands/delete'
require_relative 'commands/version'
require_relative 'utils/logger'

module TodoCli
  class CLI < Thor
    def self.exit_on_failure?
      true
    end

    desc 'add TEXT', 'Add a new todo item'
    option :priority, type: :string, default: 'medium', desc: 'Priority: high, medium, or low'
    def add(text)
      command = Commands::Add.new(text, options)
      command.execute
    end

    desc 'list', 'List all todos'
    option :all, type: :boolean, aliases: '-a', desc: 'Show completed todos too'
    option :format, type: :string, default: 'table', desc: 'Output format: table or json'
    def list
      command = Commands::List.new(options)
      command.execute
    end

    desc 'complete ID', 'Mark a todo as completed'
    def complete(id)
      command = Commands::Complete.new(id, options)
      command.execute
    end

    desc 'delete ID', 'Delete a todo'
    def delete(id)
      command = Commands::Delete.new(id, options)
      command.execute
    end

    desc 'version', 'Display version information'
    option :json, type: :boolean, desc: 'Output version info as JSON'
    def version
      command = Commands::Version.new(options)
      command.execute
    end
  end
end

TodoCli::CLI.start(ARGV) if __FILE__ == $PROGRAM_NAME
