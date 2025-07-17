import inspect
import ply.lex as lex
import ply.yacc as yacc
from .configs import tokens, precedence, symbol_op_table, symbol_op_func_table


class Assembler:
    t_ADD = r"\+"
    t_SUB = r"-"
    t_MUL = r"\*"
    t_DIV = r"/"
    t_REM = r"%"
    t_SHR = r">>"
    t_SHL = r"<<"
    t_AND = r"&"
    t_OR = r"\|"
    t_XOR = r"\^"
    t_NOT = r"~"
    # t_NEG = r"-"
    t_LPAREN = r"\("
    t_RPAREN = r"\)"
    t_EQUALS = r"="
    t_COLON = r":"
    t_COMMA = r","
    t_ignore = " \t"

    def t_COMMENT(self, t):
        r"\#.*"
        pass

    def t_NUMBER(self, t):
        r"\d+"
        t.value = int(t.value)
        return t

    def t_ID(self, t):
        r"[a-zA-Z_][a-zA-Z_0-9]*"
        if t.value == "return":
            t.type = "RETURN"
        elif t.value == "def":
            t.type = "DEF"
        return t

    def t_newline(self, t):
        r"\n+"
        t.lexer.lineno += len(t.value)

    def t_error(self, t):
        print(f"Illegal character '{t.value[0]}'")
        t.lexer.skip(1)

    def __init__(self):
        self.assembly = []
        self.variables = {}
        self.temp_counter = 0
        self.tokens = tokens
        self.precedence = precedence
        self.lexer = lex.lex(module=self)
        self.parser = yacc.yacc(module=self)
        self.symbol_op_table = symbol_op_table
        self.symbol_op_table_inv = {v: k for k, v in self.symbol_op_table.items()}
        self.symbol_op_func_table = symbol_op_func_table

        for key in self.symbol_op_func_table:
            if key not in self.symbol_op_table:
                raise ValueError(f"symbol_op_func_table missing key: {key}")

    def new_temp(self):
        var_name = f"r{self.temp_counter}"
        self.temp_counter += 1
        return var_name

    def p_function(self, p):
        "function : DEF ID LPAREN params RPAREN COLON statements RETURN expression"
        p[0] = p[9]
        temp_var = self.variables.get(p[9], p[9])
        self.assembly.append(f"OUT {temp_var}")

    def p_params(self, p):
        """params : params COMMA param
        | param
        | empty"""
        if len(p) > 2:
            p[0] = p[1] + [p[3]]
        elif p[1] is not None:
            p[0] = [p[1]]
        else:
            p[0] = []

    def p_param(self, p):
        "param : ID"
        self.variables[p[1]] = self.new_temp()
        p[0] = p[1]
        self.assembly.append(f"VAR {p[1]} {self.variables[p[1]]}")

    def p_empty(self, p):
        "empty :"
        pass

    def p_statements(self, p):
        """statements : statements statement
        | statement"""
        pass

    def p_statement(self, p):
        "statement : ID EQUALS expression"
        self.variables[p[1]] = p[3]  # Store the temp_var for the ID

    def p_expression_binop(self, p):
        """expression : expression ADD expression
        | expression SUB expression
        | expression MUL expression
        | expression DIV expression
        | expression REM expression
        | expression SHR expression
        | expression SHL expression
        | expression AND expression
        | expression OR expression
        | expression XOR expression"""

        temp_var = self.new_temp()
        op = self.symbol_op_table_inv[p[2]]
        p[1] = self.variables.get(p[1], p[1])
        p[3] = self.variables.get(p[3], p[3])
        self.assembly.append(f"{op} {p[1]} {p[3]} {temp_var}")
        p[0] = temp_var

    def p_expression_uniop(self, p):
        """expression : NOT expression"""
        temp_var = self.new_temp()
        op = self.symbol_op_table_inv[p[1]]
        p[2] = self.variables.get(p[2], p[2])
        self.assembly.append(f"{op} {p[2]} {temp_var}")
        p[0] = temp_var

    def p_expression_group(self, p):
        "expression : LPAREN expression RPAREN"
        p[0] = p[2]

    def p_expression_number(self, p):
        "expression : NUMBER"
        temp_var = self.new_temp()
        self.assembly.append(f"MOV {p[1]} {temp_var}")
        p[0] = temp_var

    def p_expression_id(self, p):
        "expression : ID"
        p[0] = p[1]

    def p_error(self, p):
        raise SyntaxError(f"Syntax error at '{p.value}'")

    def parse(self, input_string):
        self.assembly = []
        self.temp_counter = 0
        self.variables = {}
        _ = self.parser.parse(input_string, lexer=self.lexer)
        return self.assembly

    def code_wrapper(self, func):
        """a python decorator to wrap a function and parse it as input_string"""

        def wrapper(*args, **kwargs):
            return func(*args, **kwargs)

        # Get the source code of the function as a string
        source_lines = inspect.getsourcelines(func)[0]

        # remove the indentation white spaces
        indent = len(source_lines[0]) - len(source_lines[0].lstrip())
        source_lines = [line[indent:] for line in source_lines]

        source_string = "".join(source_lines[1:])  # first line is wrapper itself
        wrapper.source_string = source_string
        wrapper.assembly = self.parse(source_string)
        return wrapper

    def compute_assembly(self, assemblys):
        """execute assembly instructions & return a output value"""
        stack = []

        for assembly in assemblys:
            if assembly.startswith("#"):
                continue
            op, *args, var = assembly.split()
            if op == "OUT":
                return next(v for v in stack if v[1] == var)[0]
            if op == "MOV":
                stack.append((int(args[0]), var))
            if op == "VAR":
                # Todo
                print("VAR")
            elif len(args) == 2:
                # binary operation
                a = next(v for v in stack if v[1] == args[0])[0]
                b = next(v for v in stack if v[1] == args[1])[0]
                stack.append((self.symbol_op_func_table[op](a, b), var))
            elif len(args) == 1:
                # unary operation
                a = next((v for v in stack if v[1] == args[0]))[0]
                stack.append((self.symbol_op_func_table[op](a), var))
