require_relative '../utils/database'

module TodoCli
  module Commands
    class Complete
      def initialize(id, _options = {})
        @id = id.to_i
      end

      def execute
        db = Utils::Database.new

        begin
          todo = db.find_todo(@id)

          if todo.nil?
            puts "Todo ##{@id} not found"
            return Result.new(success: false, message: 'Todo not found')
          end

          if todo['completed'] == 1
            puts "Todo ##{@id} is already completed"
            return Result.new(success: false, message: 'Already completed')
          end

          if db.complete_todo(@id)
            $stdout.puts "✓ Completed todo ##{@id}: #{todo['text']}"
            $stdout.flush
            Result.new(success: true, message: 'Todo completed')
          else
            Result.new(success: false, message: 'Failed to complete todo')
          end
        rescue StandardError => e
          Result.new(success: false, message: e.message)
        ensure
          db.close
        end
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
