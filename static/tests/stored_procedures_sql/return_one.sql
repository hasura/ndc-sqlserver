CREATE PROCEDURE dbo.ReturnOne
AS
BEGIN
  SET NOCOUNT ON;
  BEGIN TRANSACTION;
  SELECT 1 AS Result;
  COMMIT;
END;
