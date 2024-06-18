SELECT ISNULL(
  (SELECT
     s.name AS [schema],
     p.name AS [name],
     (
       SELECT
         pr.parameter_id,
         pr.name,
         t.name AS [type],
         pr.max_length,
         pr.is_output,
         pr.is_nullable
         FROM sys.parameters pr
              INNER JOIN sys.types t ON pr.user_type_id = t.user_type_id
        WHERE pr.object_id = p.object_id
        ORDER BY pr.parameter_id
        FOR JSON PATH
     ) AS arguments
     FROM sys.procedures p
     INNER JOIN sys.schemas s ON p.schema_id = s.schema_id
     FOR JSON PATH
  ), '[]')
