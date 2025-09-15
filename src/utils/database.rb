require 'sqlite3'
require 'fileutils'

module TodoCli
  module Utils
    class Database
      DB_FILE = File.join(File.dirname(__FILE__), '..', '..', 'tmp', 'todocli.db')

      def initialize
        ensure_database_exists
        @db = SQLite3::Database.new(DB_FILE)
        @db.results_as_hash = true
        setup_tables
      end

      def add_todo(text, priority = 'medium')
        @db.execute(
          'INSERT INTO todos (text, priority, completed, created_at) VALUES (?, ?, ?, datetime("now"))',
          [text, priority, 0]
        )
        @db.last_insert_row_id
      end

      def list_todos(show_completed: false)
        sql = if show_completed
                'SELECT * FROM todos ORDER BY completed ASC, created_at DESC'
              else
                'SELECT * FROM todos WHERE completed = 0 ORDER BY
                 CASE priority
                   WHEN "high" THEN 1
                   WHEN "medium" THEN 2
                   WHEN "low" THEN 3
                 END, created_at DESC'
              end
        @db.execute(sql)
      end

      def complete_todo(id)
        @db.execute('UPDATE todos SET completed = 1, completed_at = datetime("now") WHERE id = ?', [id])
        @db.changes.positive?
      end

      def delete_todo(id)
        @db.execute('DELETE FROM todos WHERE id = ?', [id])
        @db.changes.positive?
      end

      def find_todo(id)
        @db.execute('SELECT * FROM todos WHERE id = ?', [id]).first
      end

      def close
        @db&.close
      end

      private

      def ensure_database_exists
        FileUtils.mkdir_p(File.dirname(DB_FILE))
      end

      def setup_tables
        @db.execute <<-SQL
          CREATE TABLE IF NOT EXISTS todos (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            text TEXT NOT NULL,
            priority TEXT DEFAULT 'medium',
            completed INTEGER DEFAULT 0,
            created_at TEXT NOT NULL,
            completed_at TEXT
          )
        SQL
      end
    end
  end
end
