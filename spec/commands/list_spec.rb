require 'spec_helper'
require_relative '../../src/commands/list'
require_relative '../../src/commands/add'
require 'tempfile'

RSpec.describe TodoCli::Commands::List do
  let(:temp_db) { Tempfile.new('test_todos.db') }

  before do
    # Mock the database file path to use a temporary file
    stub_const('TodoCli::Utils::Database::DB_FILE', temp_db.path)
  end

  after do
    temp_db.unlink
  end

  describe '#execute' do
    context 'with no todos' do
      it 'displays no todos message' do
        command = described_class.new

        expect { command.execute }.to output(/No todos found/).to_stdout
      end
    end

    context 'with todos' do
      before do
        # Add some test todos
        capture_stdout { TodoCli::Commands::Add.new('Buy groceries', priority: 'high').execute }
        capture_stdout { TodoCli::Commands::Add.new('Walk the dog', priority: 'medium').execute }
        capture_stdout { TodoCli::Commands::Add.new('Read book', priority: 'low').execute }
      end

      it 'displays todos in priority order' do
        command = described_class.new
        output = capture_stdout { command.execute }

        expect(output).to include('Buy groceries')
        expect(output).to include('Walk the dog')
        expect(output).to include('Read book')
        expect(output).to include('[HIGH]')
        expect(output).to include('[MEDIUM]')
        expect(output).to include('[LOW]')
      end

      it 'shows todo count summary' do
        command = described_class.new
        output = capture_stdout { command.execute }

        expect(output).to include('Total: 3 (3 pending, 0 completed)')
      end

      it 'outputs valid JSON format when requested' do
        command = described_class.new(format: 'json')
        output = capture_stdout { command.execute }

        json_data = JSON.parse(output)
        expect(json_data).to be_an(Array)
        expect(json_data.length).to eq(3)
        expect(json_data.first).to have_key('text')
        expect(json_data.first).to have_key('priority')
      end
    end

    it 'returns a successful result object' do
      command = described_class.new

      result = nil
      capture_stdout { result = command.execute }

      expect(result).to be_success
    end
  end
end
