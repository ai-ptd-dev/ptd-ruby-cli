require_relative '../utils/database'
require 'rainbow'

module TodoCli
  module Commands
    class List
      def initialize(options = {})
        @show_completed = options[:all] || false
        @format = options[:format] || 'table'
      end

      def execute
        db = Utils::Database.new

        begin
          todos = db.list_todos(show_completed: @show_completed)

          if todos.empty?
            puts 'No todos found.'
            return Result.new(success: true, message: 'No todos')
          end

          case @format
          when 'json'
            output_json(todos)
          else
            output_table(todos)
          end

          Result.new(success: true, message: "Listed #{todos.length} todos")
        rescue StandardError => e
          Result.new(success: false, message: e.message)
        ensure
          db.close
        end
      end

      private

      def output_table(todos)
        puts "\nTodos:"
        puts '─' * 80

        todos.each do |todo|
          status = todo['completed'] == 1 ? '✓' : '○'
          priority_color = case todo['priority']
                           when 'high' then :red
                           when 'medium' then :yellow
                           when 'low' then :green
                           else :white
                           end

          priority_text = Rainbow("[#{todo['priority'].upcase}]").color(priority_color)

          if todo['completed'] == 1
            puts "#{status} #{todo['id'].to_s.rjust(3)} #{Rainbow(todo['text']).color(:bright_black)} #{priority_text}"
          else
            puts "#{status} #{todo['id'].to_s.rjust(3)} #{todo['text']} #{priority_text}"
          end
        end

        puts '─' * 80
        completed_count = todos.count { |t| t['completed'] == 1 }
        pending_count = todos.count { |t| t['completed'].zero? }
        puts "Total: #{todos.length} (#{pending_count} pending, #{completed_count} completed)"
      end

      def output_json(todos)
        require 'json'
        puts JSON.pretty_generate(todos)
      end

      class Result
        attr_reader :success, :message

        def initialize(success:, message:)
          @success = success
          @message = message
        end

        def success?
          @success
        end
      end
    end
  end
end
