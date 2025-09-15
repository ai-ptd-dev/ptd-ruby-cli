require_relative '../utils/database'

module TodoCli
  module Commands
    class Delete
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

          if db.delete_todo(@id)
            puts "✗ Deleted todo ##{@id}: #{todo['text']}"
            Result.new(success: true, message: 'Todo deleted')
          else
            puts "Failed to delete todo ##{@id}"
            Result.new(success: false, message: 'Failed to delete todo')
          end
        rescue StandardError => e
          puts "Error: #{e.message}"
          Result.new(success: false, message: e.message)
        ensure
          db&.close
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
